use crate::tr;
use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>);
pub(super) type HelpGroup = (&'static str, Vec<HelpEntry>);

fn help_entry(key: impl Into<String>, label: &'static str) -> HelpEntry {
    (key.into(), Cow::Borrowed(label))
}

fn keybind_label(bindings: &crate::config::ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| "unset".to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind]) -> String {
    if bindings.is_empty() {
        "unset".to_string()
    } else if bindings.len() == 9 {
        let first = &bindings[0].label;
        if first.ends_with('1') {
            format!("{}1..9", first.trim_end_matches('1'))
        } else {
            bindings
                .iter()
                .map(|binding| binding.label.clone())
                .collect::<Vec<_>>()
                .join(" / ")
        }
    } else {
        bindings
            .iter()
            .map(|binding| binding.label.clone())
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let mut groups = Vec::new();

    groups.push((
        tr!("keybind.section.global"),
        vec![
            help_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                tr!("keybind.section.prefix_mode"),
            ),
            help_entry(keybind_label(&kb.help), tr!("keybind.action.keybinds")),
            help_entry(keybind_label(&kb.settings), tr!("keybind.action.settings")),
            help_entry(keybind_label(&kb.detach), tr!("keybind.action.detach")),
            help_entry(keybind_label(&kb.reload_config), tr!("keybind.action.reload_config")),
            help_entry(
                keybind_label(&kb.open_notification_target),
                tr!("keybind.action.open_notification_target"),
            ),
        ],
    ));

    groups.push((
        tr!("keybind.section.navigation"),
        vec![
            help_entry("esc", tr!("keybind.action.back")),
            help_entry(
                format!(
                    "{} / {}",
                    keybind_label(&kb.navigate.workspace_up),
                    keybind_label(&kb.navigate.workspace_down)
                ),
                tr!("keybind.action.workspace_list"),
            ),
            help_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    keybind_label(&kb.navigate.pane_left),
                    keybind_label(&kb.navigate.pane_down),
                    keybind_label(&kb.navigate.pane_up),
                    keybind_label(&kb.navigate.pane_right)
                ),
                tr!("keybind.action.move_focus"),
            ),
            help_entry("tab / shift+tab", tr!("keybind.action.cycle_pane")),
            help_entry("enter", tr!("keybind.action.open_workspace")),
            help_entry("1..9", tr!("keybind.action.switch_workspace")),
        ],
    ));

    let workspace_tab = vec![
        help_entry(keybind_label(&kb.workspace_picker), tr!("keybind.action.workspace_navigation")),
        help_entry(keybind_label(&kb.goto), tr!("keybind.action.session_navigator")),
        help_entry(keybind_label(&kb.new_workspace), tr!("keybind.action.new_workspace")),
        help_entry(keybind_label(&kb.new_worktree), tr!("keybind.action.new_worktree")),
        help_entry(keybind_label(&kb.open_worktree), tr!("keybind.action.open_worktree")),
        help_entry(
            keybind_label(&kb.remove_worktree),
            tr!("keybind.action.delete_worktree_checkout"),
        ),
        help_entry(keybind_label(&kb.rename_workspace), tr!("keybind.action.rename_workspace")),
        help_entry(keybind_label(&kb.close_workspace), tr!("keybind.action.close_workspace")),
        help_entry(keybind_label(&kb.previous_workspace), tr!("keybind.action.previous_workspace")),
        help_entry(keybind_label(&kb.next_workspace), tr!("keybind.action.next_workspace")),
        help_entry(indexed_label(&kb.switch_workspace), tr!("keybind.action.switch_workspace_1_9")),
        help_entry(keybind_label(&kb.previous_agent), tr!("keybind.action.previous_agent")),
        help_entry(keybind_label(&kb.next_agent), tr!("keybind.action.next_agent")),
        help_entry(indexed_label(&kb.focus_agent), tr!("keybind.action.focus_agent_1_9")),
        help_entry(keybind_label(&kb.new_tab), tr!("keybind.action.new_tab")),
        help_entry(keybind_label(&kb.rename_tab), tr!("keybind.action.rename_tab")),
        help_entry(keybind_label(&kb.previous_tab), tr!("keybind.action.previous_tab")),
        help_entry(keybind_label(&kb.next_tab), tr!("keybind.action.next_tab")),
        help_entry(indexed_label(&kb.switch_tab), tr!("keybind.action.switch_tab_1_9")),
        help_entry(keybind_label(&kb.close_tab), tr!("keybind.action.close_tab")),
    ];
    groups.push((tr!("keybind.section.workspaces_tabs"), workspace_tab));

    let panes = vec![
        help_entry(keybind_label(&kb.split_vertical), tr!("keybind.action.split_vertical")),
        help_entry(keybind_label(&kb.split_horizontal), tr!("keybind.action.split_horizontal")),
        help_entry(keybind_label(&kb.close_pane), tr!("keybind.action.close_pane")),
        help_entry(keybind_label(&kb.rename_pane), tr!("keybind.action.rename_pane")),
        help_entry(keybind_label(&kb.edit_scrollback), tr!("keybind.action.edit_scrollback")),
        help_entry(keybind_label(&kb.copy_mode), tr!("keybind.action.copy_mode")),
        help_entry(keybind_label(&kb.zoom), tr!("keybind.action.zoom_pane")),
        help_entry(keybind_label(&kb.resize_mode), tr!("keybind.action.resize_mode")),
        help_entry(keybind_label(&kb.toggle_sidebar), tr!("keybind.action.toggle_sidebar")),
        help_entry(keybind_label(&kb.focus_pane_left), tr!("keybind.action.focus_pane_left")),
        help_entry(keybind_label(&kb.focus_pane_down), tr!("keybind.action.focus_pane_down")),
        help_entry(keybind_label(&kb.focus_pane_up), tr!("keybind.action.focus_pane_up")),
        help_entry(keybind_label(&kb.focus_pane_right), tr!("keybind.action.focus_pane_right")),
        help_entry(keybind_label(&kb.cycle_pane_next), tr!("keybind.action.cycle_pane_next")),
        help_entry(
            keybind_label(&kb.cycle_pane_previous),
            tr!("keybind.action.cycle_pane_previous"),
        ),
        help_entry(keybind_label(&kb.last_pane), tr!("keybind.action.last_pane")),
    ];
    groups.push((tr!("keybind.section.panes"), panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            tr!("keybind.section.custom"),
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed(tr!("keybind.action.custom_command"))),
                    )
                })
                .collect(),
        ));
    }

    groups
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<(usize, Line<'static>)> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = keybind_help_groups(app);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _)| key.chars().count()))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    for (group, entries) in groups {
        lines.push((
            group.len() + 1,
            Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
        ));
        for (key, label) in entries {
            let padded_key = format!(" {:<width$} ", key, width = key_width);
            let width = padded_key.chars().count() + label.chars().count();
            lines.push((
                width,
                Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
            ));
        }
        lines.push((0, Line::raw("")));
    }

    lines
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], tr!("keybind.action.keybinds"), &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(header_rows[0]),
        Some("esc"),
        tr!("button.close"),
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(
        Paragraph::new(format!(" {}", tr!("release.available_commands")))
            .style(Style::default().fg(app.palette.overlay1)),
        header_rows[1],
    );

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let body = Paragraph::new(
        keybind_help_lines(app)
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>(),
    )
    .wrap(Wrap { trim: false })
    .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", tr!("release.scroll")), Style::default().fg(app.palette.overlay0)),
            Span::styled("wheel ↑↓", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled(tr!("release.jump"), Style::default().fg(app.palette.overlay0)),
            Span::styled(" pgup / pgdn ", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled(tr!("button.close"), Style::default().fg(app.palette.overlay0)),
            Span::styled(" esc / enter ", Style::default().fg(app.palette.text)),
        ])),
        stack.footer.unwrap_or_default(),
    );
}
