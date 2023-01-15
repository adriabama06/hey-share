const config = require('./config.json');

const fs = require('fs');
const path = require('path');

var files = new Map();

function load(file, folder, map) {
    try {
        
        const content = JSON.parse(
            fs.readFileSync(
                path.join(folder, file)
            )
        );
    
        if(content.url && content.file) {
            map.set(content.url, content.file);
        }

    } catch { }
}

function loadAll(folder) {
    var new_files = new Map();

    fs.readdirSync(folder)
    .forEach((file, index) => {
        load(file, folder, new_files);
    });

    files = new_files;
}

setInterval(() => {
    loadAll(config.routes);
}, 1000);

/**
 * @returns {Map<string, string>}
 */
function getFiles() {
    return files;
}

module.exports = {
    getFiles
}