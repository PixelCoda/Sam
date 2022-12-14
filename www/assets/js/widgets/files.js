// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

class StoredFiles {
    constructor() {
        this.files = undefined;
        this.folders = [];
        this.dropbox_enabled = false;
        this.virtual_path = "/";
        this.return_path = "";
        this.opened_file = {
            open: false,
            filename: "",
        };
        this.default_storage_location = undefined;

       var ref = this;
        $.get("/api/settings/key:default_storage_location/value", function( data ) {
            ref.default_storage_location = data;
        });
        


    }

    refreshFiles(){
        var ref = this;
        if(!this.dropbox_enabled){
            this.checkDropbox();
        }
        $.get("/api/services/storage/files", function( data ) {
            var has_updates = false;
            if(data !== ref.files){
                ref.files = data;
                has_updates = true;
            }
            if(!ref.virtual_path.includes("/Dropbox/") && has_updates){
                ref.refreshHTML();
            }
        });
    }

    checkDropbox(){
        var ref = this;
        $.get("/api/services", function( data ) {
            $(data).each(function() {
                if(this.identifier == "dropbox") {
                    ref.dropbox_enabled = true;
                    ref.refreshHTML();
                } 
            });
        });
    }

    refreshHTML(){

        var html = "";
        var table = $("#stored-files-table");
        var ref = this;
        this.folders = [];

        // Opened virtual file system root
        if(this.virtual_path == "/"){


            html += `
                <tr >
                    <td class='no-controller-select'><a onclick="uploadFile()" href="#" class="glow-animation media-app-button controller-btn"><i class="fas fa-upload glow-animation media-app-button-icon round"></i></a>
                    </td>
                </tr>
            `;



            // If enable render a Dropbox folder
            if(this.dropbox_enabled){
                html += `
                    <tr onclick="openDropboxRoot()">
                        <td><i style="font-size: 20px;" class="fab fa-dropbox"></i> Dropbox</td>
                    </tr>
                `;
            }
    
    
            $(this.files).each(function(i, obj) {

                if(obj.file_folder_tree !== null && obj.file_folder_tree !== undefined && obj.file_folder_tree.length > 1){
                    if(!ref.folders.includes(obj.file_folder_tree[0])){
                        ref.folders.push(obj.file_folder_tree[0]);
                        html += `
                            <tr onclick="openFolder('${obj.file_folder_tree[0]}')">
                                <td><i class="fas fa-folder"></i> ${obj.file_folder_tree[0]}</td>
                            </tr>
                        `;
                    }
                } else {
                    html += `
                        <tr onclick="openFile('${obj.oid}', '${obj.file_name}')">
                            <td>${iconFromName(obj.file_name)} ${obj.file_name}</td>
                        </tr>
                    `;
                }


            });
        }

        // Opened dropbox root
        if(this.virtual_path == "/Dropbox/"){
            html += `
                <tr onclick="returnToRootPath()">
                    <td><i style="font-size: 20px;" class="fas fa-caret-left"></i> Back</td>
                </tr>
            `;


            $.get("/api/services/dropbox", function( data ) {
                $(data).each(function() {
                    if(this.object_type === "folder"){
                        html += `
                            <tr onclick="openDropboxPath('${this.path}')">
                                <td>${this.path.replace("/", "")}</td>
                            </tr>
                        `;
                    } else {
                        html += `
                        <tr onclick="openDropboxFile('${this.path}', '${this.path.replace(ref.virtual_path.replace("/Dropbox", ""), "").replace("/", "")}')">
                            <td>${this.path.replace(ref.virtual_path.replace("/Dropbox", ""), "").replace("/", "")}</td>
                        </tr>
                    `;
                    }
          
                });
                table.html(html);
            });

            
        }

        // Opened dropbox directory
        if(this.virtual_path !== "/Dropbox/" && this.virtual_path.includes("/Dropbox/") && !this.opened_file.open){
            html += `
                <tr onclick="returnToRootPath()">
                    <td><i style="font-size: 20px;" class="fas fa-caret-left"></i> Back</td>
                </tr>
            `;


            $.get("/api/services/dropbox?path=" + this.virtual_path.replace("/Dropbox", ""), function( data ) {
                $(data).each(function() {
                    if(this.object_type === "folder"){
                        html += `
                            <tr onclick="openDropboxPath('${this.path}')">
                                <td>${this.path.replace(ref.virtual_path.replace("/Dropbox", ""), "").replace("/", "")}</td>
                            </tr>
                        `;
                    } else {
                        html += `
                            <tr onclick="openDropboxFile('${this.path}', ${this.path.replace(ref.virtual_path.replace("/Dropbox", ""), "").replace("/", "")})">
                                <td>${this.path.replace(ref.virtual_path.replace("/Dropbox", ""), "").replace("/", "")}</td>
                            </tr>
                        `;
                    }
          
                });
                table.html(html);
            });

            
        }

        // Opened dropbox file
        if(this.virtual_path !== "/Dropbox/" && this.virtual_path.includes("/Dropbox/") && this.opened_file.open){
         


            html += `
                <tr onclick="returnToRootPath()">
                    <td><i style="font-size: 20px;" class="fas fa-caret-left"></i> Back</td>
                </tr>
            `;


            html += `
                <tr>
                    <td class='no-controller-select'><center>${this.opened_file.filename}</center></td>
                </tr>
            `;
                



          

            
        }

        // Opened file from SQL storage
        if(this.virtual_path.includes("oid:") && this.opened_file.open){



            html += `
                <tr onclick="returnToPreviousPath()">
                    <td class='no-controller-select'>
                        <button class="btn btn-sm btn-secondary">
                            <i class="fas fa-arrow-left"></i>
                        </button>

                        <button class="btn btn-sm btn-danger">
                            <i class="fas fa-trash"></i>
                        </button>
                    </td>
                </tr>
            `;


            html += `
                <tr>
                    <td class='no-controller-select'><center>${this.opened_file.filename}</center></td>
                </tr>
            `;
            

  
            var x = this.opened_file.filename;

            // If opend file is an image:
            if(x.includes(".png") || x.includes(".jpg") || x.includes(".jpeg")){
                html += `<tr><td class='no-controller-select'><img style="width: 100%;" src='/files/${this.virtual_path.replace("oid:", "")}'></img></td></tr>`;
            }


            if(x.includes(".mp4") ){
                html += `<tr><td class='no-controller-select'>
                
                <video width="320" height="240" controls>
                    <source src="/files/${this.virtual_path.replace("oid:", "")}" type="video/mp4">
                </video>
                
                </td></tr>`;
            }




            
        }

        // Opened virtual file system path
        if(this.virtual_path !== "/" && !this.opened_file.open && !this.virtual_path.includes("/Dropbox/")){



            html += `
                <tr onclick="returnToPreviousPath()">
                    <td><i style="font-size: 20px;" class="fas fa-caret-left"></i> Back</td>
                </tr>
            `;

            var split = this.virtual_path.split("/");
            console.log(split);


            $(this.files).each(function(i, obj) {
                if(obj.file_folder_tree !== null && obj.file_folder_tree !== undefined && obj.file_folder_tree.length > split.length-1){
                    if(!ref.folders.includes(obj.file_folder_tree[split.length-1])){


                        if(obj.file_folder_tree.includes(split[split.length-1])){
                            ref.folders.push(obj.file_folder_tree[split.length-1]);
                            html += `
                                <tr onclick="openFolder('${obj.file_folder_tree[split.length-1]}')">
                                    <td><i class="fas fa-folder"></i> ${obj.file_folder_tree[split.length-1]}</td>
                                </tr>
                            `;    
                        }

 




                    }
                } else {
                    if(obj.file_folder_tree !== null && obj.file_folder_tree !== undefined && obj.file_folder_tree.includes(split[split.length-1])){
                        html += `
                            <tr onclick="openFile('${obj.oid}', '${obj.file_name}')">
                                <td>${iconFromName(obj.file_name)} ${obj.file_name}</td>
                            </tr>
                        `;
                    }

                }


            });

        }

        table.html(html);
    }

    add(file) {
        if(this.files == undefined){
            this.files = [];
        } else {
            
        }
        this.remove(file.oid);
        this.files.push(file);
    }

    remove(oid) {
        var i = 0;
        while (i < this.files.length) {
          if (this.files[i].oid === oid) {
            this.files.splice(i, 1);
          } else {
            ++i;
          }
        }
      }

    get(oid){
        return this.files.filter(function (item) {
            return item.oid === oid;
        })[0];
    }
}



class StoredFile {
    constructor(object) {
        this.oid = object.oid;
        this.file_name = object.file_name;
        this.file_type = object.file_type;
        this.file_folder_tree = object.file_folder_tree;
        this.storage_location_oid = object.storage_location_oid;
        this.created_at = object.created_at;
        this.updated_at = object.updated_at;
    }


}


var stored_files = new StoredFiles();
stored_files.refreshFiles();
function handleFileClick(oid){
    $(stored_files.files).each(function(i, obj) {
        if(obj.oid === oid){
            newPopWindow(`/api/services/storage/file/${obj.oid}`, obj.file_name, 0, 0, 1000, 1000)
        }
    });
}




function openDropboxRoot(){
    stored_files.virtual_path = "/Dropbox/";
    stored_files.opened_file.open = false;
    stored_files.opened_file.filename = "";
    stored_files.refreshHTML();
}

function returnToRootPath(){
    stored_files.virtual_path = "/";
    stored_files.opened_file.open = false;
    stored_files.opened_file.filename = "";
    stored_files.refreshHTML();
}

function openDropboxPath(path){
    stored_files.virtual_path = "/Dropbox" + path;
    stored_files.opened_file.open = false;
    stored_files.opened_file.filename = "";
    stored_files.refreshHTML();
}

function openDropboxFile(path, name){
    stored_files.virtual_path = "/Dropbox" + path;
    stored_files.opened_file.open = true;
    stored_files.opened_file.filename = name;
    stored_files.refreshHTML();
}

function openFile(oid, name){
    stored_files.return_path = stored_files.virtual_path;
    stored_files.virtual_path = "oid:" + oid;
    stored_files.opened_file.open = true;
    stored_files.opened_file.filename = name;
    stored_files.refreshHTML();
}

function returnToPreviousPath(){
    if(stored_files.return_path === stored_files.virtual_path){
        var split = stored_files.virtual_path.split("/");
        console.log(split.length);

        if(split.length === 2 || split.length === 1){
            return returnToRootPath();
        }


        var popped = split.splice(-1);
        console.log(popped);
        if(popped.length > 0){
            var npath = popped.join("/");
            stored_files.return_path = npath;
        } else {
            stored_files.return_path = "/";
        }

    }
    if(stored_files.return_path.length < 1){
        stored_files.return_path = "/";
    }

    stored_files.virtual_path = stored_files.return_path;
    stored_files.opened_file.open = false;
    stored_files.opened_file.filename = "";
    stored_files.refreshHTML();
}

function openFolder(path){
    if(stored_files.virtual_path == "/"){
        stored_files.virtual_path = "";
    }
    stored_files.return_path = stored_files.virtual_path;
    stored_files.virtual_path = stored_files.virtual_path + "/" + path;
    stored_files.opened_file.open = false;
    stored_files.opened_file.filename = "";
    stored_files.refreshHTML();
}


function iconFromName(name){
    if(name.includes(".png") || name.includes(".jpg") || name.includes(".jpeg")){
        return `<i style="font-size: 20px;" class="fas fa-file-image"></i>`;
    }
}