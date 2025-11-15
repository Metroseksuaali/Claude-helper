---
description: Show detailed token usage breakdown for current session
---

# Detailed Token Usage

Shows comprehensive breakdown of token usage for your current Claude Code session.

## Usage

```
/token-usage
```

## Information Displayed

### Current Session
- Input tokens (with cost)
- Output tokens (with cost)
- Cache reads (free)
- Cache writes (with cost)
- Total tokens and cost

### Usage Limits
- 5-hour block usage (updated every 5 seconds)
- 7-day total usage
- Burn rate (tokens/hour)
- Estimated time until limit

### Session Details
- Session duration
- Messages sent
- Tool calls made
- Average tokens per message

## Example Output

```
╔═══════════════════════════════════════════════════════════╗
║ Token Usage Breakdown                                     ║
╠═══════════════════════════════════════════════════════════╣
║ Current Session:                                          ║
║   Input tokens:     32,150 ($0.48)                       ║
║   Output tokens:    13,050 ($0.20)                       ║
║   Cache reads:       8,420 (free)                        ║
║   Cache writes:      4,210 ($0.05)                       ║
║ ────────────────────────────────────────────────────────  ║
║   Total:           45,200 ($0.68)                         ║
╠═══════════════════════════════════════════════════════════╣
║ Usage Limits:                                             ║
║   5-hour:  ████████████░░░░░░░░ 70% (14k/20k)           ║
║   7-day:   ██████████░░░░░░░░░░ 65% (130k/200k)         ║
║                                                           ║
║   Burn rate: 2.3k tokens/hour                            ║
║   Time to limit: ~2.6 hours                              ║
╠═══════════════════════════════════════════════════════════╣
║ Session Details:                                          ║
║   Duration: 2h 15m                                        ║
║   Messages: 45                                            ║
║   Tool calls: 128                                         ║
║   Avg tokens/message: 1,004                              ║
╚═══════════════════════════════════════════════════════════╝
```

## Execution

```bash
claude-helper status --detailed
```
