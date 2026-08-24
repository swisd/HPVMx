const fs = require('fs');
const createQemuModule = require('./qemu-system-x86_64.js');

// 1. Hook into Emscripten lifecycle before execution begins
const qemuConfig = {
    arguments: [
        "-machine", "q35",
        "-cpu", "max",
        "-drive", "if=pflash,format=raw,readonly=on,file=/code.fd",
        "-drive", "if=pflash,format=raw,file=/vars.fd",
        "-drive", "id=disk0,file=/disk.vhd,format=vpc,if=none",
        "-device", "ide-hd,drive=disk0,bus=ide.0"
    ],
    preRun: [function() {
        // 2. Read your Windows host files and map them into the WASM sandboxed memory
        const codeFd = fs.readFileSync('./code.fd');
        const varsFd = fs.readFileSync('./vars.fd');
        const diskVhd = fs.readFileSync('./web/disk.vhd'); // Make sure your disk is small (<100MB) if loading synchronously!

        qemuConfig.FS.createDataFile('/', 'code.fd', codeFd, true, false);
        qemuConfig.FS.createDataFile('/', 'vars.fd', varsFd, true, true);
        qemuConfig.FS.createDataFile('/', 'disk.vhd', diskVhd, true, true);

        console.log("All EFI maps and VHD images successfully loaded into virtual RAM.");
    }],
    print: function(text) { console.log('QEMU stdout: ' + text); },
    printErr: function(text) { console.error('QEMU stderr: ' + text); }
};

// 3. Fire up the emulator engine
createQemuModule(qemuConfig).then(() => {
    console.log("QEMU WebAssembly Engine initiated.");
});
