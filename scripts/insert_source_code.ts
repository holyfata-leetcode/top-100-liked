import * as fs from 'fs';
import * as path from 'path';

const readmePath = path.resolve(__dirname, '../README.md');
const srcDir = path.resolve(__dirname, '../src');

function insertSourceCode() {
    let readme = fs.readFileSync(readmePath, 'utf-8');
    const regex = /<!-- insert_source_code src=(.*?) -->[\s\S]*?<!-- insert_source_code -->/g;

    readme = readme.replace(regex, (_, srcRelPath) => {
        const srcPath = path.resolve(srcDir, path.basename(srcRelPath));
        if (!fs.existsSync(srcPath)) {
            return `<!-- insert_source_code src=${srcRelPath} -->\n\`\`\`\n// 源文件不存在：${srcRelPath}\n\`\`\`\n<!-- insert_source_code -->`;
        }
        const code = fs.readFileSync(srcPath, 'utf-8');
        const ext = path.extname(srcPath).slice(1) || '';
        return `<!-- insert_source_code src=${srcRelPath} -->\n\`\`\`${ext}\n${code}\n\`\`\`\n<!-- insert_source_code -->`;
    });

    fs.writeFileSync(readmePath, readme, 'utf-8');
}

insertSourceCode();
