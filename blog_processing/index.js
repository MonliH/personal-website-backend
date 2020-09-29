import remarkable from "remarkable";
import yaml from "js-yaml";
import fs from "fs";
import path from "path";
import hljs from "highlight.js";

let { Remarkable } = remarkable;
let md = new Remarkable({
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(lang, str).value;
      } catch (err) {}
    }

    try {
      return hljs.highlightAuto(str).value;
    } catch (err) {}

    return ""; // use external default escaping
  }
});

async function ls(from, to) {
  const dir = await fs.promises.opendir(from);
  for await (const dirent of dir) {
      fs.readFile(path.join(from, dirent.name), (err, data) => {
          if (err) {
              throw new Error(err);
          } else {
              let post = yaml.load(data.toString());
              post.contents = md.render(post.contents, yaml.FAILSAFE_SCHEMA);
              let d = post.date;
              let date_string = `${d.getFullYear()}-${d.getMonth().toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;

              delete post.date;

              fs.writeFile(path.join(to, dirent.name), `${yaml.dump(post)}date: ${date_string}`, () => {
                console.log(`\x1b[32mFile \x1b[34m${path.join(from, dirent.name)}\x1b[32m processed.\x1b[0m`);
              });
          }
      });
  }
}

ls("./assets/blog/entries", "./assets/blog/__processed_entries").catch(console.error);
