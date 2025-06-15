import * as fs from 'fs';
import * as path from 'path';

const srcDir = path.resolve(__dirname, '../src');
const readmePath = path.resolve(__dirname, '../README.md');

// 读取 src 目录下的文件数量，排除 main.rs
const files = fs.readdirSync(srcDir).filter(file => {
  const stat = fs.statSync(path.join(srcDir, file));
  return stat.isFile() && file !== 'main.rs';
});
const count = files.length;

// 计算进度百分比
const percent = Math.floor((count / 100) * 100);

// 读取 README.md 内容
let readme = fs.readFileSync(readmePath, 'utf-8');

// 自动替换两个注释之间的内容
readme = readme.replace(
  /(<!-- update_progress -->)[\s\S]*?(<!-- update_progress -->)/,
  `<!-- update_progress -->\n
![](https://img.shields.io/badge/编程语言-Rust-dea584)
![](https://img.shields.io/badge/进度-${percent}%25-blue)\n
<!-- update_progress -->`
);

// 写回 README.md
fs.writeFileSync(readmePath, readme);

console.log(`已更新进度为 ${percent}%`);

