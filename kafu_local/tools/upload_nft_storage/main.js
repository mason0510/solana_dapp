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
async function upload_dir(storage,project_dir,name,decoration_id,decoration_url,start,end){
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
            decoration_id:decoration_id,
            decoration_url:decoration_url,
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

async function upload_room_dir(storage,project_dir,name,decoration_id,decoration_url,start,end,level){
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
        }

        let metadata = {
            name: name + " #" + id,
            symbol: '',
            description: '',
            image: 'https://'+image_cid+'.ipfs.nftstorage.link',
            external_url:"",
            decoration_id:decoration_id,
            decoration_url:decoration_url,
            attributes: [{trait_type: "id",value: id},{trait_type: "level",value: level}]
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
    //Kcoin
    //let kcoin_uri = await upload_kin(storage,"./resource/kin/","Kin")
    //console.log("Kin_uri ",kcoin_uri);

    const storage = new NFTStorage({ endpoint, token })
    var upload_cids = []

    //todo: config
    //room
    for (let level = 1; level < 2; level++) {
        let name = "LEVEL"+level+"-ROOM";
        let collection_uri = await upload_collection(storage,"./resource/room/level"+level,name + " collection")
        let token_uri = await upload_room_dir(storage, "./resource/room/level"+level, name, "","",0, 200,level);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,start:0,end:200})
        console.log("complete upload %s",name)
    }

    //avatar_frame

    //todo： from config file
    let frame_infos = [
        "Moon@403302703029036824@https://ali-oss-app.ttchat.com/decoration/lottie/package/ramadan_travel.zip@0@240",
        "Around the Universe@363562918706429428@https://ttchat-ugc.oss-accelerate.aliyuncs.com/decoration/lottie/package/universe.zip@0@100",
        "Crescent@405053786696984244@https://app-cos.kafumena.com/decoration/lottie/package/eid_al_fitr.zip@0@40",
        "Blue Wings@402124619923857448@https://app-cos.kafumena.com/decoration/lottie/package/blue_wings.zip@0@20",
        "Supercar@398981968009634034@https://app-cos.kafumena.com/decoration/lottie/package/supercar.zip@0@400",
        "Ghoul@372736664948187758@https://app-cos.kafumena.com/decoration/lottie/package/ghoul.zip@0@200",
        "Golden dragon@372736352841638510@https://app-cos.kafumena.com/decoration/lottie/package/sovereign.zip@0@100",
    ]
    for (let i = 0; i < 7; i++) {
        let [name,decoration_id,decoration_url,start,end] = frame_infos[i].split('@');
        let collection_uri = await upload_collection(storage,"./resource/avatar_frame/"+name,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/avatar_frame/"+name, name, decoration_id,decoration_url,parseInt(start), parseInt(end));
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,start:parseInt(start),end:parseInt(end)})
        console.log("complete upload %s",name)
    }

    //vehicle
    let vehicle_infos = [
       "Black car@400298145453452561@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_Black_car_220705204937_4_app.zip@0@240",
        "Carriage@421170513029638029@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_carriage1_220518211509_1_app.zip@0@100",
        "Eagle flying@422156587122042811@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_ying_220822155227_2_app.zip@0@40",
        "Wolf@417072891293607179@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_wolfnew_220718142446_1_app.zip@0@20",
        "Red motorcycle@400298246066414503@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_Red_motorcycle_220705204453_1_app.zip@0@300",
        "Cruise Ship@402148572738363432@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_cruiseship_220705142630_2_app.zip@0@150",
        "Lion@428986713256241223@https://app-cos.kafumena.com/decoration/lottie/package/enter_room_Lion_221008183913_1_app.zip@0@50",
    ];
    for (let i = 0; i < 7; i++) {
        let [name,decoration_id,decoration_url,start,end] = vehicle_infos[i].split('@');
        let collection_uri = await upload_collection(storage,"./resource/vehicle/"+name,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/vehicle/"+name, name , decoration_id,decoration_url,parseInt(start), parseInt(end));
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,start:parseInt(start),end:parseInt(end)})
        console.log("complete upload %s",name)
    }
    await fs.promises.writeFile("./resource/upload_cids.json",JSON.stringify(upload_cids,null,"\t"));

}
main()