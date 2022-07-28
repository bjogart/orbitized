use crate::palette::with_alpha;
use crate::palette::Palette;
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(b: &mut ThemeBuilder, p: &Palette) {
    b.add_workspace_rules(
        &[
            "editor.background",
            "editorInlayHint.background",
            "editorInlayHint.parameterBackground",
            "editorInlayHint.typeBackground",
        ],
        p.base_bg,
    );
    b.add_workspace_rules(&["editor.foreground", "foreground"], p.base_fg);
    b.add_workspace_rule("icon.foreground", p.sub_fg);

    b.add_workspace_rule("editor.lineHighlightBackground", p.base_bg);

    b.add_workspace_rules(&["editor.selectionBackground", "selection.background"], p.em_bg);

    b.add_workspace_rules(&["editorCursor.foreground", "terminalCursor.foreground"], p.em_fg);
    b.add_workspace_rules(&["editorCursor.background", "terminalCursor.background"], p.em_bg);

    b.add_workspace_rules(&["activityBar.background", "sideBar.background"], p.base_bg);
    b.add_workspace_rule("activityBar.foreground", p.em_fg);
    b.add_workspace_rule("activityBar.inactiveForeground", p.sub_fg);

    b.add_workspace_rule("statusBar.foreground", p.sub_fg);
    b.add_workspace_rules(
        &["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        p.base_bg,
    );

    b.add_workspace_rules(
        &["statusBarItem.errorForeground", "statusBarItem.warningForeground"],
        p.base_bg,
    );
    b.add_workspace_rule("statusBarItem.errorBackground", p.red);
    b.add_workspace_rule("statusBarItem.warningBackground", p.orange);

    b.add_workspace_rules(
        &["badge.foreground", "activityBarBadge.foreground", "extensionBadge.remoteForeground"],
        p.base_bg,
    );
    b.add_workspace_rules(
        &["badge.background", "activityBarBadge.background", "extensionBadge.remoteBackground"],
        p.magenta,
    );

    b.add_workspace_rule("editorLineNumber.foreground", p.sub_fg);
    b.add_workspace_rule("editorLineNumber.activeForeground", p.base_fg);

    b.add_workspace_rule("editorWidget.background", p.base_bg);
    b.add_workspace_rule("editorWidget.foreground", p.base_fg);

    b.add_workspace_rule("input.foreground", p.base_fg);
    b.add_workspace_rule("input.background", p.base_bg);
    b.add_workspace_rule("input.placeholderForeground", p.sub_fg);
    b.add_workspace_rule("inputOption.activeBackground", p.em_bg);
    b.add_workspace_rule("inputOption.activeForeground", p.em_fg);

    b.add_workspace_rules(
        &[
            "list.focusBackground",
            "list.activeSelectionBackground",
            "list.inactiveSelectionBackground",
            "list.hoverBackground",
        ],
        p.em_bg,
    );
    b.add_workspace_rules(
        &[
            "list.focusForeground",
            "list.activeSelectionForeground",
            "list.inactiveSelectionForeground",
            "list.hoverForeground",
            "list.highlightForeground",
        ],
        p.em_fg,
    );

    b.add_workspace_rules(&["editorGroupHeader.tabsBackground", "tab.inactiveBackground", "tab.activeBackground"], p.base_bg);
    b.add_workspace_rule("tab.inactiveForeground", p.sub_fg);
    b.add_workspace_rule("tab.activeForeground", p.em_fg);

    b.add_workspace_rules(&["titleBar.activeBackground", "titleBar.inactiveBackground"], p.base_bg);
    b.add_workspace_rule("titleBar.activeForeground", p.base_fg);
    b.add_workspace_rule("titleBar.inactiveForeground", p.sub_fg);

    b.add_workspace_rule("quickInputList.focusBackground", p.em_bg);
    b.add_workspace_rule("quickInputList.focusForeground", p.em_fg);

    b.add_workspace_rule("breadcrumb.foreground", p.sub_fg);
    b.add_workspace_rule("breadcrumb.focusForeground", p.base_fg);

    b.add_workspace_rule("terminal.foreground", p.base_fg);
    b.add_workspace_rule("terminal.ansiBlack", p.em_bg);
    b.add_workspace_rule("terminal.ansiRed", p.red);
    b.add_workspace_rule("terminal.ansiGreen", p.green);
    b.add_workspace_rule("terminal.ansiYellow", p.yellow);
    b.add_workspace_rule("terminal.ansiBlue", p.blue);
    b.add_workspace_rule("terminal.ansiMagenta", p.violet);
    b.add_workspace_rule("terminal.ansiCyan", p.cyan);
    b.add_workspace_rule("terminal.ansiWhite", p.em_fg);
    b.add_workspace_rule("terminal.ansiBrightBlack", p.sub_fg);
    b.add_workspace_rule("terminal.ansiBrightRed", p.orange);
    b.add_workspace_rule("terminal.ansiBrightGreen", p.green);
    b.add_workspace_rule("terminal.ansiBrightYellow", p.yellow);
    b.add_workspace_rule("terminal.ansiBrightBlue", p.blue);
    b.add_workspace_rule("terminal.ansiBrightMagenta", p.magenta);
    b.add_workspace_rule("terminal.ansiBrightCyan", p.cyan);
    b.add_workspace_rule("terminal.ansiBrightWhite", p.em_fg);

    b.add_workspace_rule("focusBorder", p.sub_fg);

    b.add_workspace_rules(
        &[
            "editorInlayHint.foreground",
            "editorInlayHint.parameterForeground",
            "editorInlayHint.typeForeground",
        ],
        p.sub_fg,
    );
    b.add_workspace_rules(
        &[
            "editorInlayHint.background",
            "editorInlayHint.parameterBackground",
            "editorInlayHint.typeBackground",
        ],
        with_alpha(p.base_bg, 0x00),
    );

    b.add_workspace_rule("errorLens.hintBackground", with_alpha(p.cyan, 0x20));
    b.add_workspace_rule("errorLens.hintForeground", p.cyan);
    b.add_workspace_rule("errorLens.infoBackground", with_alpha(p.green, 0x20));
    b.add_workspace_rule("errorLens.infoForeground", p.green);
    b.add_workspace_rule("errorLens.warningBackground", with_alpha(p.orange, 0x20));
    b.add_workspace_rule("errorLens.warningForeground", p.orange);
    b.add_workspace_rule("errorLens.errorBackground", with_alpha(p.red, 0x20));
    b.add_workspace_rule("errorLens.errorForeground", p.red);

    b.add_workspace_rules(
        &["editorLightBulb.foreground", "editorLightBulbAutoFix.foreground"],
        p.yellow,
    );

    b.add_workspace_rule("editorBracketHighlight.foreground1", p.blue);
    b.add_workspace_rule("editorBracketHighlight.foreground2", p.green);
    b.add_workspace_rule("editorBracketHighlight.foreground3", p.violet);
    b.add_workspace_rule("editorBracketHighlight.foreground4", p.cyan);
    b.add_workspace_rule("editorBracketHighlight.foreground5", p.yellow);
    b.add_workspace_rule("editorBracketHighlight.foreground6", p.magenta);

    b.add_workspace_rule("editorBracketHighlight.unexpectedBracket.foreground", p.red);

    b.add_workspace_rule("diffEditor.insertedTextBackground", with_alpha(p.green, 0x30));
    b.add_workspace_rule("diffEditor.removedTextBackground", with_alpha(p.red, 0x30));
    b.add_workspace_rule("diffEditor.insertedLineBackground", with_alpha(p.green, 0x20));
    b.add_workspace_rule("diffEditor.removedLineBackground", with_alpha(p.red, 0x20));
}

fn syntax_highlighting(b: &mut ThemeBuilder, p: &Palette) {
    b.add_rules(
        &[
            Semantic("keyword"),
            Textmate("keyword"),
            Textmate("storage"),
            Textmate("variable.language.self"),
            Textmate("variable.language.this"),
            Textmate("keyword.type.go"),
        ],
        p.green,
    );
    b.add_rules(&[Semantic("*.controlFlow"), Textmate("keyword.control")], p.magenta);

    b.add_rules(&[Semantic("function.trait"), Semantic("method.trait")], p.violet);

    b.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("builtinType"),
            Textmate("entity.name.type"),
            Textmate("keyword.type"),
            Textmate("storage.type.cs"),
            Textmate("storage.type.java"),
            Textmate("storage.type.boolean.go"),
            Textmate("storage.type.byte.go"),
            Textmate("storage.type.error.go"),
            Textmate("storage.type.numeric.go"),
            Textmate("storage.type.rune.go"),
            Textmate("storage.type.string.go"),
            Textmate("storage.type.uintptr.go"),
        ],
        p.yellow,
    );
    b.add_rules(
        &[
            Semantic("interface"),
            Semantic("typeAlias.trait"),
            Semantic("typeParameter"),
            Textmate("entity.name.type.interface"),
            Textmate("entity.name.type.type-parameter"),
        ],
        p.violet,
    );

    b.add_rules(
        &[Semantic("variable.constant"), Semantic("variable.static"), Textmate("constant")],
        p.cyan,
    );
    b.add_rule(Semantic("variable.constant.trait"), p.violet);

    b.add_rules(&[Semantic("number"), Textmate("constant.numeric")], p.cyan);
    b.add_rules(&[Semantic("string"), Semantic("characterLiteral"), Textmate("string")], p.cyan);

    b.add_rules(
        &[
            Semantic("property"),
            Textmate("entity.name.variable.field"),
            Textmate("entity.name.variable.property"),
            Textmate("variable.other.member"),
            Textmate("variable.other.object.property"),
            Textmate("variable.other.readwrite.instance"),
            Textmate("support.type.property-name"),
        ],
        p.blue,
    );

    b.add_rules(
        &[
            Semantic("enumMember"),
            Semantic("boolean"),
            Textmate("entity.name.variable.enum-member"),
            Textmate("constant.other.enum"),
            Textmate("variable.other.enummember"),
            Textmate("entity.name.type.option"),
            Textmate("entity.name.type.result"),
        ],
        p.cyan,
    );

    b.add_rules(
        &[
            Semantic("macro"),
            Textmate("entity.name.function.macro"),
            Textmate("entity.name.function.preprocessor"),
        ],
        p.magenta,
    );

    b.add_rules(
        &[
            Semantic("formatSpecifier"),
            Semantic("escapeSequence"),
            Textmate("constant.other.placeholder"),
            Textmate("punctuation.definition.interpolation"),
            Textmate("punctuation.section.embedded"),
            Textmate("constant.character.escape"),
        ],
        p.red,
    );

    b.add_rule(Semantic("lifetime"), p.blue);

    b.add_rules(&[Semantic("comment"), Textmate("comment")], (p.sub_fg, FontStyle::Italic));

    b.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.line.documentation"),
            Textmate("comment.block.documentation"),
            Textmate("comment.block.javadoc"),
        ],
        (p.base_fg, FontStyle::Italic),
    );

    b.add_rules(
        &[
            Semantic("*.attribute"),
            Textmate("entity.name.function.decorator"),
            Textmate("storage.type.annotation"),
            Textmate("punctuation.definition.annotation"),
        ],
        p.sub_fg,
    );

    b.add_rule(Textmate("keyword.operator"), p.base_fg);

    b.add_rule(Semantic("unresolvedReference"), p.red);

    b.add_rule(Textmate("markup.deleted"), p.red);
    b.add_rule(Textmate("markup.inserted"), p.green);
    b.add_rule(Textmate("markup.changed"), p.blue);
    b.add_rule(Textmate("meta.diff"), p.sub_fg);

    b.add_rule(Textmate("comment.line.number-sign.git-commit"), p.sub_fg);
    b.add_rule(Textmate("invalid.deprecated.line-too-long.git-commit"), p.magenta);
    b.add_rule(Textmate("invalid.illegal.line-too-long.git-commit"), p.red);

    b.add_rule(Semantic("*.unsafe"), (p.red, FontStyle::Bold));
    b.add_rule(Semantic("*.mutable"), FontStyle::Underline);
    b.add_rule(Semantic("*.consuming"), FontStyle::Italic);
    b.add_rule(Semantic("*.public.declaration"), FontStyle::Bold);
}
