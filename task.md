## Things to fix

### 25/12
- editor part not scrollable
- scroll in preview comes in between and not on side
- character count in editor still comes up differently in gray


### Update 28/12
- made editor scrollable by using `ScrollAread::vertical()`
- Scroll in preview which used to come in between now comes to side by adding auto_shrink. ScrollArea only used required / text space. But now it uses full available space, CentralPanel.
- For character count, if I remove ui.label its ok but need to figure out a better way.



## To implement



## Approach:

### 25/12
- First approach used egui hortizontal and vertical splits to create editor and preview panes. But preview wasn't coming properly on side.

- Then used `egui::ScrollArea` to create scrollable area for preview. And also tried using egui columns to create side by side panes. This also didn't work as expected.

- FInally used `egui::SidePanel` for editor and `egui::CentralPanel` for preview. This worked as expected.


### 28/12

- To render markdown, using pulldown-cmark crate. It breaks texts into events and then we can match those events to render appropriate text in preview pane.
    - Events like: start of heading, end of heading, text, code, etc.
- when using ui.label is adds elements to new line. Instead we collect RichText elements in a Vec and use ui.horizontal_wrapped with 0 spacing in between to render them in same line.

