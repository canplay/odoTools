/*
 Navicat Premium Data Transfer

 Source Server         : odo
 Source Server Type    : MongoDB
 Source Server Version : 40203
 Source Host           : 192.168.1.33:27714
 Source Schema         : website

 Target Server Type    : MongoDB
 Target Server Version : 40203
 File Encoding         : 65001

 Date: 26/03/2022 03:19:17
*/


// ----------------------------
// Collection structure for carousel
// ----------------------------
db.getCollection("carousel").drop();
db.createCollection("carousel");

// ----------------------------
// Documents of carousel
// ----------------------------
db.getCollection("carousel").insert([ {
    _id: ObjectId("623dcc88a35300006c0028c4"),
    img: "imgs/63f658e886c20210427133247165.jpg",
    url: "",
    delete: false
} ]);
db.getCollection("carousel").insert([ {
    _id: ObjectId("623dcc8fa35300006c0028c5"),
    img: "imgs/29b3ed6294020210427134409396.jpg",
    url: "",
    delete: false
} ]);
db.getCollection("carousel").insert([ {
    _id: ObjectId("623dcc98a35300006c0028c6"),
    img: "imgs/7b1ef13252520210427135930988.jpg",
    url: "",
    delete: false
} ]);
db.getCollection("carousel").insert([ {
    _id: ObjectId("623dcc9fa35300006c0028c7"),
    img: "imgs/e79d80d010120210427134146260.jpg",
    url: "",
    delete: false
} ]);

// ----------------------------
// Collection structure for config
// ----------------------------
db.getCollection("config").drop();
db.createCollection("config");

// ----------------------------
// Documents of config
// ----------------------------
db.getCollection("config").insert([ {
    _id: ObjectId("623e059ca35300006c0028d4"),
    name: "测试区服",
    "login_server": "192.168.1.33:9888",
    delete: false
} ]);

// ----------------------------
// Collection structure for downloads
// ----------------------------
db.getCollection("downloads").drop();
db.createCollection("downloads");

// ----------------------------
// Documents of downloads
// ----------------------------
db.getCollection("downloads").insert([ {
    _id: ObjectId("623dccc8a35300006c0028c9"),
    title: "Clinet",
    desc: "Descriptions",
    url: "http://192.168.1.33/downloads/client_20220313.7z",
    delete: false
} ]);
db.getCollection("downloads").insert([ {
    _id: ObjectId("623dcccea35300006c0028ca"),
    title: "Microsoft Visual C++ Runtime",
    desc: "",
    url: "",
    delete: false
} ]);
db.getCollection("downloads").insert([ {
    _id: ObjectId("623dccd3a35300006c0028cb"),
    title: "Microsoft .Net Framework Runtime",
    desc: "",
    url: "",
    delete: false
} ]);
db.getCollection("downloads").insert([ {
    _id: ObjectId("623dccd8a35300006c0028cc"),
    title: "NVIDIA",
    desc: "",
    url: "",
    delete: false
} ]);
db.getCollection("downloads").insert([ {
    _id: ObjectId("623dccdda35300006c0028cd"),
    title: "AMD",
    desc: "",
    url: "",
    delete: false
} ]);

// ----------------------------
// Collection structure for news
// ----------------------------
db.getCollection("news").drop();
db.createCollection("news");

// ----------------------------
// Documents of news
// ----------------------------
db.getCollection("news").insert([ {
    _id: ObjectId("623dcc72a35300006c0028c2"),
    title: "welcome",
    date: "2022-3-13",
    type: "announcements",
    content: "<div>test content</div>",
    delete: false
} ]);
db.getCollection("news").insert([ {
    _id: ObjectId("623dce6ba35300006c0028d3"),
    title: "welcome",
    date: "2022-3-13",
    type: "events",
    content: "<div>test content</div>",
    delete: false
} ]);

// ----------------------------
// Collection structure for shop
// ----------------------------
db.getCollection("shop").drop();
db.createCollection("shop");

// ----------------------------
// Documents of shop
// ----------------------------
db.getCollection("shop").insert([ {
    _id: ObjectId("623dcd0da35300006c0028cf"),
    preview: "imgs/no-img.png",
    name: "hot test",
    cash: 200,
    type: "hot",
    desc: "123",
    delete: false
} ]);
db.getCollection("shop").insert([ {
    _id: ObjectId("623dcd1ea35300006c0028d0"),
    preview: "imgs/no-img.png",
    name: "vip test",
    cash: 30,
    type: "vip",
    desc: "123",
    delete: false
} ]);
db.getCollection("shop").insert([ {
    _id: ObjectId("623dcd2ea35300006c0028d1"),
    preview: "imgs/no-img.png",
    name: "items test",
    cash: 20,
    type: "item",
    desc: "123",
    delete: false
} ]);
db.getCollection("shop").insert([ {
    _id: ObjectId("623dcd3ba35300006c0028d2"),
    preview: "imgs/no-img.png",
    name: "outfit test",
    cash: 150,
    type: "outfit",
    desc: "123",
    delete: false
} ]);

// ----------------------------
// Collection structure for version
// ----------------------------
db.getCollection("version").drop();
db.createCollection("version");

// ----------------------------
// Documents of version
// ----------------------------
db.getCollection("version").insert([ {
    _id: ObjectId("623dcce6a35300006c0028ce"),
    launcher: "1.0.0.0",
    client: "1.0.0.0",
    resource: "1.0.0.0",
    "client_url": "",
    "launcher_url": "",
    "resource_url": ""
} ]);
