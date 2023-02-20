import 'package:carousel_slider/carousel_slider.dart';
import 'package:flutter/material.dart';
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
    return const MaterialApp(
      themeMode: ThemeMode.dark,
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> with WindowListener {
  final List<String> entries = <String>['测试新闻', '测试新闻', '测试新闻', '测试新闻', '测试新闻'];
  final List<int> colorCodes = <int>[600, 500, 400, 300, 200];

  @override
  void initState() {
    windowManager.addListener(this);
    super.initState();
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
                    child: Padding(
                      padding: const EdgeInsets.all(8),
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
                                        decoration: BoxDecoration(
                                          image: DecorationImage(
                                            fit: BoxFit.fill,
                                            image: NetworkImage(
                                              'http://1.13.22.82:51530/imgs/slide/$i.jpg',
                                            ),
                                          ),
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
                                itemCount: entries.length,
                                itemBuilder: (BuildContext context, int index) {
                                  return SizedBox(
                                    height: 50,
                                    child: Padding(
                                      padding: const EdgeInsets.fromLTRB(
                                          20, 0, 20, 0),
                                      child: Row(
                                        children: [
                                          Text(
                                            '${entries[index]}$index',
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
                  child: Expanded(
                    child: Row(
                      children: [
                        Expanded(
                          flex: 6,
                          child: Column(
                            children: const [
                              TextField(
                                style: TextStyle(color: Colors.white),
                                decoration: InputDecoration(
                                  border: OutlineInputBorder(),
                                  enabledBorder: OutlineInputBorder(
                                    borderSide: BorderSide(color: Colors.white),
                                  ),
                                  labelText: '用户名',
                                  labelStyle: TextStyle(
                                      color: Colors.white, fontFamily: '微软雅黑'),
                                ),
                              ),
                              SizedBox(
                                height: 8,
                              ),
                              TextField(
                                style: TextStyle(color: Colors.white),
                                obscureText: true,
                                decoration: InputDecoration(
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
                                  Color.fromARGB(255, 255, 0, 255)),
                            ),
                            child: const Text(
                              "设置",
                              style:
                                  TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                            ),
                            onPressed: () => {},
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
                                  Colors.amberAccent),
                            ),
                            child: const Text(
                              "注册",
                              style:
                                  TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                            ),
                            onPressed: () => {},
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
                              style:
                                  TextStyle(fontSize: 24, fontFamily: '微软雅黑'),
                            ),
                            onPressed: () => {},
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(
              width: double.infinity,
              height: 45,
              child: Card(
                color: Color.fromARGB(120, 0, 0, 0),
                child: Padding(
                  padding: EdgeInsets.all(8),
                  child: Expanded(
                    child: Text(
                      "正在检查更新...",
                      style: TextStyle(color: Colors.white, fontFamily: '微软雅黑'),
                    ),
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
