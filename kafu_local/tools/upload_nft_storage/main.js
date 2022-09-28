import  fs from 'fs'
import { NFTStorage, File } from 'nft.storage'
import path from 'path'

const endpoint = 'https://api.nft.storage' // the default
const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDBmQzc0MTY4RjUxZTk0NjU3ODE4N0EwMTBmMjdBMjllOTFmQzZjYmEiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY1OTYwMzkyNzI3MywibmFtZSI6InNvbGFuYSJ9.cNyVRbXYsObBPVvXDax2kYwMRtROHEFim9hVdfI-0gg' // your API key from https://nft.storage/manage

async function upload_kin(storage,project_dir,name){
    console.log("0001")
    let metadata_path = path.join(project_dir,"json","kin.json");
    let image_data = fs.readFileSync(path.join(project_dir,"image/0.png"));
    let image_cid = await storage.storeBlob(new Blob([image_data]))
    console.log("collection image %s cid %s",name,image_cid);
    let metadata = {
        name: name,
        symbol: '',
        description: '',
        image: 'https://'+image_cid+'.ipfs.nftstorage.link',
        external_url:"",
        attributes: []
    };
    fs.writeFileSync(metadata_path,JSON.stringify(metadata));
    let file_data = new File([  fs.readFileSync(metadata_path)],"kin.json");
    const metadata_dir_cid = await storage.storeDirectory([file_data])
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link/kin.json"
}

async function upload_collection(storage,project_dir,name){
    console.log("0001")
    let metadata_path = path.join(project_dir,"json","collection.json");
    let image_data = fs.readFileSync(path.join(project_dir,"image/0.png"));
    let image_cid = await storage.storeBlob(new Blob([image_data]))
    console.log("collection image %s cid %s",name,image_cid);
    let metadata = {
        name: name,
        symbol: '',
        description: '',
        image: 'https://'+image_cid+'.ipfs.nftstorage.link',
        external_url:"",
        attributes: []
    };
    fs.writeFileSync(metadata_path,JSON.stringify(metadata));
    let file_data = new File([  fs.readFileSync(metadata_path)],"collection.json");
    const metadata_dir_cid = await storage.storeDirectory([file_data])
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link/collection.json"
}

//房间的直接从start开始铸造新的一批
//非房间的需要png命名从上一批的last-id续上
async function upload_dir(storage,project_dir,name,start,end){
    var files_data = []
    var image_cid = null;
    let image_files = fs.readdirSync(path.join(project_dir,"image"));
    for (let id = start; id < end; id++) {
        let json_name = id+".json";
        let metadata_path = path.join(project_dir,"json",json_name);
        //房间的都一样，目前都是一个每个类型的图片可以做成多个nft的,所以image只上传一次而且是0.png而不是对应的{id}.png
        let image_path = path.join(project_dir,"image",image_files[0]);

        if (!image_cid ){
            let image_data =  fs.readFileSync(image_path);
            image_cid = await storage.storeBlob(new Blob([image_data]))
            //console.log("%s cid %s",image_path,image_cid);
        }

        let metadata = {
            name: name + " #" + id,
            symbol: '',
            description: '',
            image: 'https://'+image_cid+'.ipfs.nftstorage.link',
            external_url:"",
            attributes: [{trait_type: "id",value: id}]
        };
        fs.writeFileSync(metadata_path,JSON.stringify(metadata));
        let file_data = new File([  fs.readFileSync(metadata_path)],json_name);
        files_data.push(file_data);
    }
    const metadata_dir_cid = await storage.storeDirectory(files_data)
    //console.log("name %s dir cid: https://%s.ipfs.nftstorage.link",name,metadata_dir_cid);
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link"
}

async function main() {
    console.log("000__001")
    const storage = new NFTStorage({ endpoint, token })
    var upload_cids = []
    //Kcoin
    let kcoin_uri = await upload_kin(storage,"./resource/kin/","Kin")
    console.log("Kin_uri ",kcoin_uri);

    //todo: config
    //room
    for (let level = 1; level < 2; level++) {
        let name = "LEVEL"+level+"-ROOM";
        let collection_uri = await upload_collection(storage,"./resource/room/level"+level,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/room/level"+level, name, 0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }

    //avatar_frame
    for (let i = 0; i < 15; i++) {
        let name = "frame"+i;
        let collection_uri = await upload_collection(storage,"./resource/avatar_frame/"+i,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/avatar_frame/"+i, name, 0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }

    //vehicle
    for (let i = 0; i < 15; i++) {
        let name = "vehicle"+i;
        let collection_uri = await upload_collection(storage,"./resource/vehicle/"+i,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/vehicle/"+i, name , 0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }
    await fs.promises.writeFile("./resource/upload_cids.json",JSON.stringify(upload_cids,null,"\t"));

}
main()