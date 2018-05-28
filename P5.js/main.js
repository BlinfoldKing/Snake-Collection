const {app, BrowserWindow} = require('electron');
  
  function createWindow () {
    win = new BrowserWindow({width: 530, height: 550});  
    win.loadFile('index.html');
  
}
  
  app.on('ready', createWindow);