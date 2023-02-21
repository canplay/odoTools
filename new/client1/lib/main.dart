import 'dart:ffi' as ffi;
import 'dart:io';

import 'package:carousel_slider/carousel_slider.dart';
import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:window_manager/window_manager.dart';
import 'package:ffi/ffi.dart';
import 'package:win32/win32.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await windowManager.ensureInitialized();
  const WindowOptions windowOptions = WindowOptions(
    size: Size(1024, 768),
    minimumSize: Size(1024, 768),
    maximumSize: Size(1024, 768),
    center: true,
    backgroundColor: Colors.transparent,
    skipTaskbar: false,
    titleBarStyle: TitleBarStyle.hidden,
  );
  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
    await windowManager.focus();
  });

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: const MyHomePage(),
      builder: FlutterSmartDialog.init(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage>
    with WindowListener, SingleTickerProviderStateMixin {
  final remoteUrl = "http://1.13.22.82:51530";

  Dio dio = Dio(
    BaseOptions(
      connectTimeout: const Duration(seconds: 5),
      receiveTimeout: const Duration(seconds: 5),
    ),
  );

  List<Map<String, dynamic>> news = [];

  String status = "正在检查网络链接...";

  late TextEditingController usernameController;
  late TextEditingController passwordController;

  late TabController tabController;
  List<Tab> myTabs = <Tab>[
    Tab(
      child: const Text(
        "登录器",
        style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
      ),
    ),
    Tab(
      child: const Text(
        "游戏",
        style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
      ),
    ),
    Tab(
      child: const Text(
        "其他",
        style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
      ),
    ),
  ];

  @override
  void initState() {
    windowManager.addListener(this);
    super.initState();
    usernameController = TextEditingController();
    passwordController = TextEditingController();
    tabController = TabController(vsync: this, length: myTabs.length);
    regProtocol();
    // getNews();
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    usernameController.dispose();
    passwordController.dispose();
    tabController.dispose();
    super.dispose();
  }

  @override
  void onWindowFocus() {
    setState(() {});
  }

  void getNews() async {
    try {
      Response resp = await dio.post("$remoteUrl/api/news/info",
          data: {
            "curPage": 0,
            "maxPage": 5,
            "sortBy": 'create_date',
            "descending": true,
            "delete": 0,
          },
          options: Options(contentType: "application/json"));

      if (resp.data["status"] == 1) {
        for (var element in resp.data["msg"]) {
          setState(() {
            news.add(element);
          });
        }
      }
    } on DioError catch (e) {
      if (e.response != null) {
        print(e.response!.data);
        print(e.response!.headers);
        print(e.response!.requestOptions);

        setState(() {
          status = e.response!.data;
        });
      } else {
        print(e.requestOptions);
        print(e.message);

        setState(() {
          status = e.message!;
        });
      }
    }
  }

  void regProtocol() {
    var hKey = calloc<HANDLE>();

    try {
      if (RegCreateKey(
              HKEY_CLASSES_ROOT, "BDOLauncher".toNativeUtf16(), hKey) !=
          ERROR_SUCCESS) {
        throw Exception("Can't create registry key 'BDOLauncher'");
      }

      var reg = "\"${Directory.current.path}\\launcher.exe\"".toNativeUtf16();
      if (RegSetKeyValue(hKey.value, "".toNativeUtf16(),
              "URL Protocol".toNativeUtf16(), REG_SZ, reg, reg.length * 2) !=
          ERROR_SUCCESS) {
        throw Exception("Can't set registry key 'URL Protocol'");
      }

      reg = "BOO Launcher Protocol".toNativeUtf16();
      if (RegSetValue(
              hKey.value, "".toNativeUtf16(), REG_SZ, reg, reg.length) !=
          ERROR_SUCCESS) {
        throw Exception("Can't set registry key 'BOO Launcher Protocol'");
      }

      RegCloseKey(hKey.value);
      free(hKey);

      hKey = calloc<HANDLE>();

      if (RegCreateKey(HKEY_CLASSES_ROOT,
              "BDOLauncher\\DefaultIcon".toNativeUtf16(), hKey) !=
          ERROR_SUCCESS) {
        throw Exception("Can't create registry key 'BDOLauncher\\DefaultIcon'");
      }

      reg = "\"${Directory.current.path}\\launcher.exe\",0".toNativeUtf16();
      if (RegSetValue(
              hKey.value, "".toNativeUtf16(), REG_SZ, reg, reg.length) !=
          ERROR_SUCCESS) {
        throw Exception("Can't set registry key 'BOO Launcher Protocol'");
      }

      RegCloseKey(hKey.value);
      free(hKey);

      hKey = calloc<HANDLE>();

      if (RegCreateKey(HKEY_CLASSES_ROOT,
              "BDOLauncher\\shell\\open\\command".toNativeUtf16(), hKey) !=
          ERROR_SUCCESS) {
        throw Exception("Can't create registry key 'BDOLauncher\\DefaultIcon'");
      }

      reg =
          "\"${Directory.current.path}\\launcher.exe\" \"%1\"".toNativeUtf16();
      if (RegSetValue(
              hKey.value, "".toNativeUtf16(), REG_SZ, reg, reg.length) !=
          ERROR_SUCCESS) {
        throw Exception("Can't set registry key 'BOO Launcher Protocol'");
      }
    } finally {
      free(hKey);
    }
  }

  void runGame(String username, String password) {
    SmartDialog.showLoading(
        msg: "正在启动游戏...", displayTime: const Duration(seconds: 10));

    final r = ShellExecute(
        0,
        "open".toNativeUtf16(),
        "${Directory.current.parent.path}\\bin64\\BlackDesert64.exe"
            .toNativeUtf16(),
        "$username,$password".toNativeUtf16(),
        "${Directory.current.parent.path}\\bin64".toNativeUtf16(),
        SW_SHOW);

    switch (r) {
      case 0:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，内存不足");
        break;
      case 2:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，文件名错误");
        break;
      case 3:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，文件夹路径错误");
        break;
      case 11:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，执行文件无效");
        break;
      case 26:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，共享错误");
        break;
      case 27:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，文件名不完全或无效");
        break;
      case 28:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，超时");
        break;
      case 31:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败，没有关联的启动程序");
        break;
      default:
        SmartDialog.dismiss();
        SmartDialog.showToast("启动游戏失败");
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(kWindowCaptionHeight),
        child: WindowCaption(
          brightness: Brightness.dark,
          title: Row(
            children: [
              Image.asset(
                'images/icon.png',
                height: 24,
              ),
              const SizedBox(
                width: 8,
              ),
              const Text('BDO Launcher'),
            ],
          ),
        ),
      ),
      body: Container(
        width: double.infinity,
        height: double.infinity,
        decoration: const BoxDecoration(
          image: DecorationImage(
            fit: BoxFit.fitHeight,
            image: AssetImage(
              'images/bg.jpg',
            ),
          ),
        ),
        child: Column(
          children: [
            Expanded(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  Expanded(
                    child: Column(
                      children: [
                        Expanded(
                          child: Card(
                            color: const Color.fromARGB(120, 0, 0, 0),
                            child: CarouselSlider(
                              options: CarouselOptions(
                                height: double.infinity,
                                autoPlay: true,
                                enlargeCenterPage: true,
                              ),
                              items: [1, 2, 3, 4, 5].map((i) {
                                return Builder(
                                  builder: (BuildContext context) {
                                    return Container(
                                      width: double.infinity,
                                      decoration: const BoxDecoration(
                                          // image: DecorationImage(
                                          //   fit: BoxFit.fill,
                                          // image: NetworkImage(
                                          //   'http://1.13.22.82:51530/imgs/slide/$i.jpg',
                                          // ),
                                          // ),
                                          ),
                                      child: Column(
                                        children: [
                                          const Spacer(
                                            flex: 6,
                                          ),
                                          Expanded(
                                            child: Container(
                                              width: double.infinity,
                                              decoration: const BoxDecoration(
                                                color: Color.fromARGB(
                                                    120, 0, 0, 0),
                                              ),
                                              child: Center(
                                                child: Text(
                                                  '图片标题$i',
                                                  style: const TextStyle(
                                                      fontSize: 24,
                                                      fontFamily: '微软雅黑',
                                                      color: Colors.white),
                                                ),
                                              ),
                                            ),
                                          ),
                                        ],
                                      ),
                                    );
                                  },
                                );
                              }).toList(),
                            ),
                          ),
                        ),
                        const SizedBox(
                          height: 8,
                        ),
                        Expanded(
                          child: Card(
                            color: const Color.fromARGB(120, 0, 0, 0),
                            child: ListView.builder(
                              itemCount: news.length,
                              itemBuilder: (BuildContext context, int index) {
                                return SizedBox(
                                  height: 50,
                                  child: Padding(
                                    padding:
                                        const EdgeInsets.fromLTRB(20, 0, 20, 0),
                                    child: Row(
                                      children: [
                                        Text(
                                          '${news[index]['title']}',
                                          style: const TextStyle(
                                              color: Colors.white,
                                              fontFamily: '微软雅黑'),
                                        ),
                                        const Spacer(),
                                        const Text(
                                          '2023-02-20',
                                          style: TextStyle(
                                              color: Colors.white,
                                              fontFamily: '微软雅黑'),
                                        ),
                                      ],
                                    ),
                                  ),
                                );
                              },
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Image.asset("images/logo.png"),
                  )
                ],
              ),
            ),
            SizedBox(
              height: 142,
              child: Card(
                color: const Color.fromARGB(120, 0, 0, 0),
                child: Padding(
                  padding: const EdgeInsets.all(10),
                  child: Row(
                    children: [
                      Expanded(
                        flex: 6,
                        child: Column(
                          children: [
                            TextField(
                              controller: usernameController,
                              style: const TextStyle(color: Colors.white),
                              decoration: const InputDecoration(
                                border: OutlineInputBorder(),
                                enabledBorder: OutlineInputBorder(
                                  borderSide: BorderSide(color: Colors.white),
                                ),
                                labelText: '用户名',
                                labelStyle: TextStyle(
                                    color: Colors.white, fontFamily: '微软雅黑'),
                              ),
                            ),
                            const SizedBox(
                              height: 8,
                            ),
                            TextField(
                              controller: passwordController,
                              style: const TextStyle(color: Colors.white),
                              obscureText: true,
                              decoration: const InputDecoration(
                                border: OutlineInputBorder(),
                                enabledBorder: OutlineInputBorder(
                                  borderSide: BorderSide(color: Colors.white),
                                ),
                                labelText: '密码',
                                labelStyle: TextStyle(
                                    color: Colors.white, fontFamily: '微软雅黑'),
                              ),
                            ),
                          ],
                        ),
                      ),
                      const SizedBox(
                        width: 8,
                      ),
                      SizedBox(
                        width: 120,
                        height: double.infinity,
                        child: ElevatedButton(
                          style: const ButtonStyle(
                            backgroundColor: MaterialStatePropertyAll<Color>(
                                Color.fromARGB(255, 255, 152, 0)),
                          ),
                          child: const Text(
                            "设置",
                            style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                          ),
                          onPressed: () {
                            SmartDialog.show(
                              builder: (context) {
                                return Container(
                                  height: 600,
                                  width: 800,
                                  decoration: BoxDecoration(
                                    color: const Color.fromARGB(
                                        250, 255, 255, 255),
                                    borderRadius: BorderRadius.circular(5),
                                  ),
                                  child: Column(
                                    children: [
                                      Expanded(
                                        child: Card(
                                          color: Colors.black,
                                          child: TabBar(
                                            controller: tabController,
                                            tabs: myTabs,
                                          ),
                                        ),
                                      ),
                                      SizedBox(
                                        height: 550,
                                        child: TabBarView(
                                          controller: tabController,
                                          children: myTabs.map((Tab tab) {
                                            return Center(
                                              child: Text(
                                                'This is ${tab.text} tab',
                                                style: const TextStyle(
                                                    fontSize: 36),
                                              ),
                                            );
                                          }).toList(),
                                        ),
                                      ),
                                    ],
                                  ),
                                );
                              },
                            );
                          },
                        ),
                      ),
                      const SizedBox(
                        width: 8,
                      ),
                      SizedBox(
                        width: 120,
                        height: double.infinity,
                        child: ElevatedButton(
                          style: const ButtonStyle(
                            backgroundColor: MaterialStatePropertyAll<Color>(
                                Color.fromARGB(255, 233, 30, 99)),
                          ),
                          child: const Text(
                            "注册",
                            style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                          ),
                          onPressed: () {
                            if (usernameController.value.text.isEmpty ||
                                passwordController.value.text.isEmpty) {
                              SmartDialog.showToast("用户名或密码不能为空");
                              return;
                            }
                          },
                        ),
                      ),
                      const SizedBox(
                        width: 8,
                      ),
                      SizedBox(
                        width: 120,
                        height: double.infinity,
                        child: ElevatedButton(
                          child: const Text(
                            "登录",
                            style: TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                          ),
                          onPressed: () {
                            if (usernameController.value.text.isEmpty ||
                                passwordController.value.text.isEmpty) {
                              SmartDialog.showToast("用户名或密码不能为空");
                              return;
                            }

                            runGame(usernameController.value.text,
                                passwordController.value.text);
                          },
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
            SizedBox(
              width: double.infinity,
              height: 45,
              child: Card(
                color: const Color.fromARGB(120, 0, 0, 0),
                child: Padding(
                  padding: const EdgeInsets.all(8),
                  child: Text(
                    status,
                    style: const TextStyle(
                        color: Colors.white, fontFamily: '微软雅黑'),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
