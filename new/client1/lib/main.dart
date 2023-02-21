import 'package:carousel_slider/carousel_slider.dart';
import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:flutter_smart_dialog/flutter_smart_dialog.dart';
import 'package:window_manager/window_manager.dart';

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

class _MyHomePageState extends State<MyHomePage> with WindowListener {
  final remoteUrl = "http://1.13.22.82:51530";

  Dio dio = Dio(
    BaseOptions(
      connectTimeout: const Duration(seconds: 5),
      receiveTimeout: const Duration(seconds: 5),
    ),
  );

  List<Map<String, dynamic>> news = [];

  String status = "正在检查网络链接...";

  TextEditingController usernameController = TextEditingController();
  TextEditingController passwordController = TextEditingController();

  @override
  void initState() {
    windowManager.addListener(this);
    super.initState();
    // getNews();
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
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
                            SmartDialog.show(builder: (context) {
                              return Container(
                                height: 600,
                                width: 800,
                                decoration: BoxDecoration(
                                  color: Colors.black,
                                  borderRadius: BorderRadius.circular(10),
                                ),
                                alignment: Alignment.center,
                                child: const Text('easy custom dialog',
                                    style: TextStyle(color: Colors.white)),
                              );
                            });
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
                            }
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
