import * as fs from "fs";

fs.writeFileSync(
  "settings.json",
  JSON.stringify({
    facility: "test",
    filePath: "test",
  })
);
