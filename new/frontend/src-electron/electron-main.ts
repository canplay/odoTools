import { app, BrowserWindow, nativeTheme, ipcMain } from 'electron';
import path from 'path';
import os from 'os';
import cp from 'child_process';
import fileStream from 'fs';

const gotTheLock = app.requestSingleInstanceLock();

if (!gotTheLock) {
  app.quit();
} else {
  app.on('second-instance', () => {
    if (mainWindow) {
      mainWindow.show();
    }
  });
}

// needed in case process is undefined under Linux
const platform = process.platform || os.platform();

try {
  if (platform === 'win32' && nativeTheme.shouldUseDarkColors === true) {
    require('fs').unlinkSync(
      path.join(app.getPath('userData'), 'DevTools Extensions')
    );
  }
} catch (_) {}

let mainWindow: BrowserWindow | undefined;

function createWindow() {
  /**
   * Initial window options
   */
  mainWindow = new BrowserWindow({
    icon: path.resolve(__dirname, 'icons/icon.png'), // tray icon
    width: 1024,
    height: 768,
    frame: false,
    center: true,
    transparent: true,
    show: false,
    useContentSize: true,
    webPreferences: {
      contextIsolation: true,
      // More info: https://v2.quasar.dev/quasar-cli-vite/developing-electron-apps/electron-preload-script
      preload: path.resolve(__dirname, process.env.QUASAR_ELECTRON_PRELOAD),
    },
  });

  mainWindow.loadURL(process.env.APP_URL);

  if (process.env.DEBUGGING) {
    // if on DEV or Production with debug enabled
    mainWindow.webContents.openDevTools();
  } else {
    // we're on production; no access to devtools pls
    mainWindow.webContents.on('devtools-opened', () => {
      mainWindow?.webContents.closeDevTools();
    });
  }

  mainWindow.on('closed', () => {
    mainWindow = undefined;

    const process = cp
      .exec('taskkill /im aria2c.exe /t /f', {
        windowsHide: true,
      })
      .on('exit', () => {
        process.kill();
      });

    delDir(app.getAppPath() + '/cached');
  });

  mainWindow.webContents.session.on('will-download', (event, item) => {
    mainWindow.webContents.send('download', item.getURL(), item.getFilename());

    event.preventDefault();
  });

  mainWindow.once('ready-to-show', () => {
    addDir(app.getAppPath() + '/cached');
    addDir(app.getAppPath() + '/downloads');

    const p = cp
      .exec('aria2c.exe --conf-path=config.conf', {
        cwd: app.getAppPath() + '/aria2',
        windowsHide: true,
      })
      .on('exit', () => {
        p.kill();
      });

    mainWindow.show();
    mainWindow.center();
    mainWindow.focus();
  });
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (mainWindow === undefined) {
    createWindow();
  }
});

ipcMain.on('launcher', (ev, arg1, arg2) => {
  const p = cp
    .exec('BlackDesert64.exe ' + arg1 + arg2, {
      cwd: app.getAppPath() + '/bin64',
    })
    .on('exit', () => {
      p.kill();
    });
});

const addDir = async (path) => {
  if (fileStream.existsSync(path)) return;
  await fileStream.mkdirSync(path);
};

const delDir = async (path) => {
  if (!fileStream.existsSync(path)) return;

  await FileListLoop(path, (file) => {
    if (!fileStream.statSync(file).isDirectory()) {
      fileStream.unlinkSync(file);
    } else {
      fileStream.rmdirSync(file);
    }
  });
};

const FileListLoop = async (path, callback) => {
  if (!fileStream.existsSync(path)) {
    return;
  }

  const files = await fileStream.readdirSync(path);

  files.forEach((file) => {
    const curPath = path + '/' + file;

    if (fileStream.statSync(curPath).isDirectory()) {
      FileListLoop(curPath, callback);
    } else {
      callback(curPath);
    }
  });

  callback(path);
};
