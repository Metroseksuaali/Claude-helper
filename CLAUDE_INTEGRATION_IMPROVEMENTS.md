# Claude-integraation parantamissuunnitelma

## Yhteenveto nykytilanteesta

### Mitä toimii ✅
- `/optimize` - Session optimointi
- `/token-usage` - Token käytön näyttö
- `/master` - Framework valmis (testaus kesken)
- `session-start` hook - Loki session aloituksesta
- `log-usage` hook - Käytön logitus
- Statusline framework asennettu

### Mitä ei toimi tai puuttuu 🚧
- Statusline käyttää mock dataa
- Ei reaaliaikaista palautetta hooksseista
- Ei visuaalista palautetta Claude Codessa
- Ei kontekstin jakamista hooksin ja slashien välillä
- Ei edistyneitä analytiikkaominaisuuksia

---

## PRIORITEETTI 1: Välittömät parannukset (Heti toteutettavissa)

### 1.1 Lisää slash-komentoja ✨

#### `/session-stats` - Session tilastot
**Tarkoitus:** Näytä current session statistiikka ilman full token breakdownia

```markdown
---
description: Quick session statistics
---

# Session Statistics

Shows quick overview of your current session.

## Usage
```
/session-stats
```

## Output
- Messages sent: 45
- Tool calls: 128
- Avg tokens/message: 1,004
- Session duration: 2h 15m
- Last optimization opportunities: 3 found

## Execution
```bash
claude-helper session-stats
```
```

**Implementaatio:** ~30 riviä koodia, lukee session tiedot


#### `/optimize-last` - Optimize viimeisimmän session
**Tarkoitus:** Analysoi VIIMEISIMMÄN session ilman parametreja

```markdown
---
description: Analyze the most recent session for optimizations
---

# Optimize Last Session

Quickly analyze your most recent Claude Code session.

## Usage
```
/optimize-last
```

Same as `/optimize` but defaults to most recent session.
```

**Implementaatio:** Alias tai wrapper `/optimize` komennolle


#### `/history` - Session historia
**Tarkoitus:** Näytä viimeisimmät sessionit

```markdown
---
description: Show recent session history
---

# Session History

Shows your recent Claude Code sessions.

## Usage
```
/history
```

## Output
- Last 10 sessions with:
  - Session ID
  - Duration
  - Total tokens
  - Key optimizations found

## Execution
```bash
claude-helper history --last 10
```
```

**Implementaatio:** ~50 riviä, listaa session tiedostot


#### `/benchmark` - Vertaa sessioita
**Tarkoitus:** Vertaa current session keskiarvoon

```markdown
---
description: Benchmark current session against your averages
---

# Session Benchmark

Compare your current session to historical averages.

## Usage
```
/benchmark
```

## Output
- Tokens vs average: +15% (you're using more than usual)
- Duration vs average: -20% (faster than usual)
- Tool calls vs average: +30% (more active session)

Helps identify if current workflow is efficient.
```

**Implementaatio:** ~100 riviä, database queries


#### `/cost-estimate` - Kustannusarvio
**Tarkoitus:** Real-time kustannusarvio remaining taskille

```markdown
---
description: Estimate cost of completing current task
---

# Cost Estimate

Estimates remaining cost based on current usage patterns.

## Usage
```
/cost-estimate
```

## Output
Based on current burn rate:
- If you continue for 1 hour: ~$0.30
- If you continue to 5h limit: ~$1.20
- Recommended: Take a break or optimize

## Execution
```bash
claude-helper cost-estimate
```
```

**Implementaatio:** ~50 riviä, matikka burn raten perusteella

---

### 1.2 Paranna hookseita 🔧

#### Ongelma nyt:
- Hookit kirjoittavat vain lokiin
- Ei palautetta käyttäjälle
- Ei älykkyyttä

#### Parannukset:

**A) Smart Alerting Hook**
```rust
async fn handle_log_usage(config: &Config) -> Result<()> {
    // Nykyinen toiminnallisuus
    let statusline = StatusLine::new(config.clone()).await?;
    let usage = statusline.get_current_usage().await?;

    // UUSI: Älykkäät hälytykset
    if usage.five_hour_percent > 80 {
        println!("⚠️  WARNING: 80% of 5-hour limit reached!");
        println!("   Consider optimizing or taking a break.");

        // Ehdota optimointia
        let analyzer = SessionAnalyzer::new(config.clone()).await?;
        let optimizations = analyzer.quick_analyze().await?;
        if !optimizations.is_empty() {
            println!("\n💡 {} optimization opportunities found!", optimizations.len());
            println!("   Run /optimize to see details");
        }
    }

    if usage.burn_rate_per_hour > 5000.0 {
        println!("🔥 HIGH BURN RATE detected: {}/hour", usage.burn_rate_per_hour);
        println!("   Consider using more specific prompts");
    }

    // Nykyinen logging
    log_to_file(&usage)?;

    Ok(())
}
```

**B) Progress Tracking Hook**
```rust
async fn handle_session_start(config: &Config) -> Result<()> {
    // Näytä edellisen session yhteenveto
    let db = Database::new(config).await?;
    if let Ok(last_session) = db.get_last_session().await {
        println!("📊 Previous session:");
        println!("   Duration: {}", last_session.duration);
        println!("   Tokens: {}", last_session.tokens);
        println!("   Cost: ${:.2}", last_session.cost);
        if last_session.optimizations > 0 {
            println!("   ✨ {} optimizations found", last_session.optimizations);
        }
    }

    // Nykyinen toiminnallisuus
    let analyzer = SessionAnalyzer::new(config.clone()).await?;
    analyzer.start_session().await?;

    Ok(())
}
```

---

### 1.3 Lisää konfiguraatiovaihtoehtoja ⚙️

```toml
[hooks]
# Näytä varoitukset konsolissa
show_warnings = true
warning_threshold_5h = 80  # prosentti
warning_threshold_burn_rate = 5000  # tokens/hour

# Näytä edellisen session yhteenveto
show_previous_session = true

# Automaattinen optimointi-ehdotus
auto_suggest_optimization = true
optimization_threshold = 3  # vähintään 3 optimointia

[slash_commands]
# Oletusarvot komennoille
default_history_count = 10
default_optimization_savings = 500  # min tokens
```

---

## PRIORITEETTI 2: Keskipitkän aikavälin parannukset

### 2.1 Session Context Sharing 🔗

**Ongelma:** Slash komennot ja hookit eivät jaa kontekstia

**Ratkaisu:** Session state tiedosto
```rust
// ~/.config/claude-helper/current-session.json
{
  "session_id": "abc123",
  "started_at": "2025-11-15T10:00:00Z",
  "messages": 45,
  "tool_calls": 128,
  "tokens_used": 45200,
  "optimizations_found": [],
  "warnings_shown": ["high_burn_rate"],
  "last_checkpoint": "2025-11-15T12:15:00Z"
}
```

**Käyttö:**
- Hookit päivittävät tätä
- Slash komennot lukevat tästä
- Mahdollistaa session-aware toiminnallisuuden

### 2.2 Smart Recommendations Engine 🧠

```rust
pub struct RecommendationEngine {
    db: Database,
    config: Config,
}

impl RecommendationEngine {
    pub async fn analyze_session(&self) -> Result<Vec<Recommendation>> {
        let mut recommendations = vec![];

        // Analysoi patterns
        let current = self.load_current_session()?;
        let history = self.db.get_session_history(30).await?;

        // Vertaa historiaan
        if current.tokens_per_message > history.avg_tokens_per_message * 1.5 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                title: "Messages are 50% larger than your average".to_string(),
                suggestion: "Consider breaking tasks into smaller parts".to_string(),
                estimated_savings: 2000,
            });
        }

        // Tool call patterns
        if current.tool_calls > history.avg_tool_calls * 2.0 {
            recommendations.push(Recommendation {
                priority: Priority::Medium,
                title: "Unusually high tool call count".to_string(),
                suggestion: "Check if you're repeating similar operations".to_string(),
                estimated_savings: 1500,
            });
        }

        // Time-based
        if current.duration > Duration::hours(3) {
            recommendations.push(Recommendation {
                priority: Priority::Low,
                title: "Long session detected".to_string(),
                suggestion: "Consider taking a break to maintain focus".to_string(),
                estimated_savings: 0,
            });
        }

        Ok(recommendations)
    }
}
```

### 2.3 Visual Feedback in Claude Code 🎨

**Ideoita:**
1. **Status emojit hooksseissa:**
   ```
   ✅ Session optimized (3 opportunities applied)
   ⚠️  Approaching 5h limit (85%)
   💰 Current cost: $0.68
   🔥 High burn rate detected
   ```

2. **Progress bars inline:**
   ```
   5h limit: ████████████░░░░░░░░ 70%
   7d limit: ██████████░░░░░░░░░░ 65%
   ```

3. **Trendit:**
   ```
   📈 Token usage trending up (+15% vs last session)
   📉 Cost trending down (-5% vs yesterday)
   ```

---

## PRIORITEETTI 3: Pitkän aikavälin visio

### 3.1 Learning System 📚

```rust
pub struct LearningEngine {
    // Oppii käyttäjän työskentelytyylistä
    // - Mihin aikaan päivästä kuluu eniten tokeneita
    // - Mitkä tehtävätyypit kuluttavat eniten
    // - Mitkä optimoinnit toimivat parhaiten
}
```

### 3.2 Predictive Analytics 🔮

```rust
pub struct PredictiveEngine {
    // Ennustaa:
    // - Milloin 5h limit tulee täyteen
    // - Kuinka paljon nykyinen tehtävä tulee maksamaan
    // - Optimaalinen aika tauolle
}
```

### 3.3 Team Features 👥

```rust
pub struct TeamMetrics {
    // Jaettava data:
    // - Team total usage
    // - Best practices
    // - Cost optimization tips
}
```

---

## Implementaatio-aikataulu

### Viikko 1: Quick Wins
- [ ] Lisää 5 uutta slash-komentoa
- [ ] Paranna hookit näyttämään palautetta
- [ ] Lisää konfiguraatiovaihtoehtoja

### Viikko 2: Session Context
- [ ] Session state tiedosto
- [ ] Smart recommendations engine
- [ ] Visual feedback parannukset

### Viikko 3: Testing & Polish
- [ ] Testaa kaikki uudet ominaisuudet
- [ ] Dokumentoi uudet komennot
- [ ] User feedback

### Viikko 4+: Advanced Features
- [ ] Learning system
- [ ] Predictive analytics
- [ ] Team features (jos kysyntää)

---

## Tekniset yksityiskohdat

### Uudet slash-komennot - Implementaatio

1. **Luo tiedostot:**
   ```bash
   .claude-templates/commands/session-stats.md
   .claude-templates/commands/optimize-last.md
   .claude-templates/commands/history.md
   .claude-templates/commands/benchmark.md
   .claude-templates/commands/cost-estimate.md
   ```

2. **Lisää CLI komennot** (`src/main.rs`):
   ```rust
   Commands::SessionStats => {
       let analyzer = SessionAnalyzer::new(config).await?;
       analyzer.show_current_stats().await?;
   }
   ```

3. **Implementoi funktiot** (`src/analyzer/mod.rs`):
   ```rust
   pub async fn show_current_stats(&self) -> Result<()> {
       // Hae current session
       // Näytä stats
   }
   ```

### Session State - Implementaatio

1. **Luo state module** (`src/session_state/mod.rs`):
   ```rust
   pub struct SessionState {
       pub session_id: String,
       pub started_at: DateTime<Utc>,
       pub metrics: SessionMetrics,
   }

   impl SessionState {
       pub fn load() -> Result<Self>;
       pub fn save(&self) -> Result<()>;
       pub fn update_metrics(&mut self, metrics: SessionMetrics) -> Result<()>;
   }
   ```

2. **Päivitä hookit käyttämään:**
   ```rust
   async fn handle_log_usage(config: &Config) -> Result<()> {
       let mut state = SessionState::load()?;
       state.update_metrics(current_metrics);
       state.save()?;

       // Smart recommendations
       if let Some(rec) = check_recommendations(&state) {
           println!("{}", rec);
       }
   }
   ```

---

## Metriikat onnistumiselle

### Käyttökokemus:
- [ ] Käyttäjä saa palautetta joka responsen jälkeen
- [ ] Varoitukset tulevat ajoissa (80% limit)
- [ ] Optimointi-ehdotukset ovat relevantteja
- [ ] Session historia on helposti saatavilla

### Tekniset:
- [ ] Kaikki slash-komennot < 100ms response time
- [ ] Hookit < 50ms overhead
- [ ] State tiedosto < 10KB
- [ ] Ei race conditioneita state päivityksissä

### Business:
- [ ] Käyttäjät säästävät keskimäärin 15% tokeneista
- [ ] 90% käyttäjistä käyttää vähintään yhtä uutta komentoa
- [ ] Positiivinen feedback > 80%

---

## Riskit ja haasteet

### Teknisiä:
1. **Race conditions:** Session state päivitykset samanaikaisista sessioneista
   - Ratkaisu: File locking tai SQLite state

2. **Performance:** Hookit hidastavat Claude responsea
   - Ratkaisu: Async background processing, < 50ms timeout

3. **Compatibility:** Toimiiko kaikissa Claude Code versioissa
   - Ratkaisu: Version detection, graceful degradation

### Käyttökokemuksellisia:
1. **Noise:** Liikaa notifikaatioita ärsyttää
   - Ratkaisu: Konfig options, smart throttling

2. **Confusion:** Liian monta komentoa sekottaa
   - Ratkaisu: Grouping, hyvä dokumentaatio

3. **Privacy:** Käyttäjät eivät halua kaikkea loggausta
   - Ratkaisu: Opt-in/opt-out options

---

## Yhteenveto

### Nopeat voitot (1-2 päivää):
1. 5 uutta slash-komentoa
2. Smart warnings hooksseissa
3. Edellisen session yhteenveto

### Keskipitkä (1-2 viikkoa):
1. Session state sharing
2. Recommendation engine
3. Visual feedback parannukset

### Pitkä (1+ kuukautta):
1. Learning system
2. Predictive analytics
3. Team features

### Prioriteetti järjestys:
1. **Välitön arvo:** Smart warnings + uudet komennot
2. **Syvempi integraatio:** Session state + recommendations
3. **Tulevaisuus:** Learning + predictions + team

Aloitetaan vaiheesta 1, testataan käyttäjillä, ja edetään feedbackin perusteella!
