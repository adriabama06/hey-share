const config = require('./config.json');

const express = require('express');
const fs = require('fs');
const path = require('path');

const app = express();

const { getFiles } = require('./files.js');

app.get('*', async (req, res) => {
    console.log(getFiles().entries(), req.path);
    const request_file = getFiles().get(req.path);

    if(!request_file) {
        return res.sendFile(
            path.join(__dirname, '404.html')
        );
    }

    res.download(path.join(__dirname, config.files, request_file));
});

app.listen(config.port, '0.0.0.0', () => {
    console.log(`Ready at: http://0.0.0.0:${config.port}/`);
});