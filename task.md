## Things to fix

### 25/12
- editor part not scrollable
- scroll in preview comes in between and not on side
- character count in editor still comes up differently in gray  

## To implement



## Approach:

### 25/12
- First approach used egui hortizontal and vertical splits to create editor and preview panes. But preview wasn't coming properly on side.

- Then used `egui::ScrollArea` to create scrollable area for preview. And also tried using egui columns to create side by side panes. This also didn't work as expected.

- FInally used `egui::SidePanel` for editor and `egui::CentralPanel` for preview. This worked as expected.

