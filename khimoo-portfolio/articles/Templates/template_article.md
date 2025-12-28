<%*
  // 現在の tm.file.title を取得
  let title = tp.file.title;

  // もし "Untitled" のままならユーザーにタイトル入力を促す
  if (title.startsWith("Untitled")) {
    title = await tp.system.prompt("ノートのタイトルを入力してください");
    // ファイル名を変更
    await tp.file.rename(title);
  }

  // frontmatter に title を書き込む
  await app.fileManager.processFrontMatter(app.workspace.getActiveFile(), (fm) => {
    fm.title = title;
    fm.home_display = true;
    fm.importance = 2;
    fm.category = "";
    fm.tags = [];
  });
%>