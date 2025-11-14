use crate::analyzer::SessionAnalyzer;
use crate::config::Config;
use crate::db::Database;
use crate::statusline::StatusLine;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::time::Duration;

enum AppTab {
    Usage,
    Optimization,
    AgentHistory,
}

pub struct App {
    config: Config,
    current_tab: AppTab,
    should_quit: bool,
    statusline: StatusLine,
    analyzer: SessionAnalyzer,
    db: Database,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let statusline = StatusLine::new(config.clone()).await?;
        let analyzer = SessionAnalyzer::new(config.clone()).await?;
        let db = Database::new(&config).await?;

        Ok(Self {
            config,
            current_tab: AppTab::Usage,
            should_quit: false,
            statusline,
            analyzer,
            db,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        super::run_tui(|terminal| {
            self.draw(terminal)?;
            self.handle_events()
        })
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Length(3), // Tabs
                    Constraint::Min(0),    // Content
                    Constraint::Length(3), // Footer
                ])
                .split(f.area());

            self.render_title(f, chunks[0]);
            self.render_tabs(f, chunks[1]);
            self.render_content(f, chunks[2]);
            self.render_footer(f, chunks[3]);
        })?;

        Ok(())
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new("Claude Helper - Interactive Dashboard")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(title, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let titles = vec!["Usage", "Optimizations", "Agent History"];
        let index = match self.current_tab {
            AppTab::Usage => 0,
            AppTab::Optimization => 1,
            AppTab::AgentHistory => 2,
        };

        let tabs = Tabs::new(titles)
            .select(index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL).title("Tabs"));

        f.render_widget(tabs, area);
    }

    fn render_content(&self, f: &mut Frame, area: Rect) {
        match self.current_tab {
            AppTab::Usage => self.render_usage_tab(f, area),
            AppTab::Optimization => self.render_optimization_tab(f, area),
            AppTab::AgentHistory => self.render_agent_history_tab(f, area),
        }
    }

    fn render_usage_tab(&self, f: &mut Frame, area: Rect) {
        // This would fetch real usage data
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Token Usage",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("5-Hour Block: ████████████░░░░ 70% (14k/20k)"),
            Line::from("7-Day Total:  █████████░░░░░░░ 65% (130k/200k)"),
            Line::from(""),
            Line::from(Span::styled(
                "Cost Information",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Burn rate: $0.15/hour"),
            Line::from("Estimated 7-day cost: $1.17"),
            Line::from(""),
            Line::from("Press 'r' to refresh"),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Usage Statistics"),
        );

        f.render_widget(paragraph, area);
    }

    fn render_optimization_tab(&self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("1. Combine git operations → Save ~600 tokens"),
            ListItem::new("2. Create test-and-build script → Save ~400 tokens"),
            ListItem::new("3. Reduce redundant file reads → Save ~900 tokens"),
            ListItem::new(""),
            ListItem::new("Total potential savings: ~1,900 tokens"),
            ListItem::new(""),
            ListItem::new("Press 'Enter' to view details"),
        ];

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Optimization Suggestions"),
        );

        f.render_widget(list, area);
    }

    fn render_agent_history_tab(&self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("💻 Code Writer Alpha - Implement auth (3.2k tokens) ✓"),
            ListItem::new("🔒 Security Auditor - Review auth (1.8k tokens) ✓"),
            ListItem::new("🧪 Test Engineer - Write tests (2.1k tokens) ✓"),
            ListItem::new("📚 Documentation Writer - API docs (1.5k tokens) ✓"),
            ListItem::new(""),
            ListItem::new("Scroll with ↑↓ arrows"),
        ];

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Recent Agent Executions"),
        );

        f.render_widget(list, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let text = Line::from(vec![
            Span::raw("Tab: "),
            Span::styled("←/→", Style::default().fg(Color::Yellow)),
            Span::raw(" | Quit: "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" | Refresh: "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
        ]);

        let paragraph =
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Controls"));

        f.render_widget(paragraph, area);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.should_quit = true;
                        return Ok(false);
                    }
                    KeyCode::Right | KeyCode::Tab => {
                        self.next_tab();
                    }
                    KeyCode::Left | KeyCode::BackTab => {
                        self.prev_tab();
                    }
                    _ => {}
                }
            }
        }

        Ok(!self.should_quit)
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Usage => AppTab::Optimization,
            AppTab::Optimization => AppTab::AgentHistory,
            AppTab::AgentHistory => AppTab::Usage,
        };
    }

    fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Usage => AppTab::AgentHistory,
            AppTab::Optimization => AppTab::Usage,
            AppTab::AgentHistory => AppTab::Optimization,
        };
    }
}
