import 'package:flutter/material.dart';
import 'screens/home_screen.dart';
import 'screens/video_screen.dart';
import 'screens/voice_screen.dart';
import 'screens/messages_screen.dart';
import 'screens/recordings_screen.dart';
import 'theme/app_theme.dart';

void main() {
  runApp(const TechubCommsApp());
}

class TechubCommsApp extends StatelessWidget {
  const TechubCommsApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Techub Comms',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.metallicTurquoise,
      home: const MainNavigation(),
    );
  }
}

class MainNavigation extends StatefulWidget {
  const MainNavigation({super.key});

  @override
  State<MainNavigation> createState() => _MainNavigationState();
}

class _MainNavigationState extends State<MainNavigation> {
  int _currentIndex = 0;

  final List<Widget> _screens = const [
    HomeScreen(),
    VideoScreen(),
    VoiceScreen(),
    MessagesScreen(),
    RecordingsScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: IndexedStack(
        index: _currentIndex,
        children: _screens,
      ),
      bottomNavigationBar: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [
              Colors.white,
              AppTheme.beige50,
            ],
          ),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.05),
              blurRadius: 10,
              offset: const Offset(0, -2),
            ),
          ],
        ),
        child: NavigationBar(
          selectedIndex: _currentIndex,
          onDestinationSelected: (index) {
            setState(() => _currentIndex = index);
          },
          backgroundColor: Colors.transparent,
          indicatorColor: AppTheme.teal500.withOpacity(0.15),
          destinations: [
            NavigationDestination(
              icon: Icon(Icons.home_outlined, color: AppTheme.metallicDark),
              selectedIcon: Icon(Icons.home, color: AppTheme.teal600),
              label: 'Home',
            ),
            NavigationDestination(
              icon: Icon(Icons.videocam_outlined, color: AppTheme.metallicDark),
              selectedIcon: Icon(Icons.videocam, color: AppTheme.teal600),
              label: 'Video',
            ),
            NavigationDestination(
              icon: Icon(Icons.call_outlined, color: AppTheme.metallicDark),
              selectedIcon: Icon(Icons.call, color: AppTheme.teal600),
              label: 'Voice',
            ),
            NavigationDestination(
              icon: Icon(Icons.chat_outlined, color: AppTheme.metallicDark),
              selectedIcon: Icon(Icons.chat, color: AppTheme.teal600),
              label: 'Messages',
            ),
            NavigationDestination(
              icon: Icon(Icons.folder_outlined, color: AppTheme.metallicDark),
              selectedIcon: Icon(Icons.folder, color: AppTheme.teal600),
              label: 'Recordings',
            ),
          ],
        ),
      ),
    );
  }
}
