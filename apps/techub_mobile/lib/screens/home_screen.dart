import 'package:flutter/material.dart';
import '../theme/app_theme.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(
          children: [
            Container(
              width: 32,
              height: 32,
              decoration: BoxDecoration(
                gradient: AppTheme.turquoiseGradient,
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Icon(Icons.video_call, color: Colors.white, size: 20),
            ),
            const SizedBox(width: 10),
            const Text('Techub Comms', style: TextStyle(fontWeight: FontWeight.bold)),
          ],
        ),
        actions: [
          IconButton(icon: const Icon(Icons.notifications_outlined), onPressed: () {}),
          IconButton(icon: const Icon(Icons.settings_outlined), onPressed: () {}),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Hero Card
            Container(
              width: double.infinity,
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                gradient: AppTheme.heroGradient,
                borderRadius: BorderRadius.circular(16),
                boxShadow: [
                  BoxShadow(
                    color: AppTheme.teal500.withOpacity(0.3),
                    blurRadius: 20,
                    offset: const Offset(0, 8),
                  ),
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Welcome to Techub',
                    style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    'Connect through voice, text and video',
                    style: TextStyle(color: Colors.white.withOpacity(0.85), fontSize: 14),
                  ),
                  const SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: () {},
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.white,
                      foregroundColor: AppTheme.teal700,
                    ),
                    child: const Text('Start a Meeting'),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),

            // Quick Actions
            const Text('Quick Actions', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(child: _ActionCard(icon: Icons.videocam, label: 'Video Call', color: AppTheme.teal500, onTap: () {})),
                const SizedBox(width: 12),
                Expanded(child: _ActionCard(icon: Icons.call, label: 'Voice Call', color: AppTheme.beige500, onTap: () {})),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(child: _ActionCard(icon: Icons.chat, label: 'Message', color: AppTheme.teal600, onTap: () {})),
                const SizedBox(width: 12),
                Expanded(child: _ActionCard(icon: Icons.group, label: 'Meeting', color: AppTheme.metallicGold, onTap: () {})),
              ],
            ),
            const SizedBox(height: 24),

            // Join Room
            Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: AppTheme.beige100),
                boxShadow: [BoxShadow(color: Colors.black.withOpacity(0.04), blurRadius: 8)],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Join a Meeting', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      Expanded(
                        child: TextField(
                          decoration: InputDecoration(
                            hintText: 'Enter room name...',
                            hintStyle: TextStyle(color: Colors.grey[400]),
                          ),
                        ),
                      ),
                      const SizedBox(width: 12),
                      ElevatedButton(onPressed: () {}, child: const Text('Join')),
                    ],
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),

            // Stats
            Row(
              children: [
                Expanded(child: _StatCard(title: 'HD', subtitle: 'Video Quality')),
                const SizedBox(width: 12),
                Expanded(child: _StatCard(title: 'E2E', subtitle: 'Encryption')),
                const SizedBox(width: 12),
                Expanded(child: _StatCard(title: '6+', subtitle: 'Participants')),
                const SizedBox(width: 12),
                Expanded(child: _StatCard(title: '24/7', subtitle: 'Available')),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _ActionCard extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;
  final VoidCallback onTap;

  const _ActionCard({required this.icon, required this.label, required this.color, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: AppTheme.beige100),
          boxShadow: [BoxShadow(color: Colors.black.withOpacity(0.04), blurRadius: 8)],
        ),
        child: Column(
          children: [
            Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                gradient: LinearGradient(colors: [color, color.withOpacity(0.7)]),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Icon(icon, color: Colors.white, size: 24),
            ),
            const SizedBox(height: 8),
            Text(label, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
          ],
        ),
      ),
    );
  }
}

class _StatCard extends StatelessWidget {
  final String title;
  final String subtitle;

  const _StatCard({required this.title, required this.subtitle});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: AppTheme.beige100),
      ),
      child: Column(
        children: [
          Text(title, style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, foreground: Paint()..shader = AppTheme.turquoiseGradient.createShader(const Rect.fromLTWH(0, 0, 100, 30)))),
          const SizedBox(height: 4),
          Text(subtitle, style: TextStyle(fontSize: 11, color: Colors.grey[500])),
        ],
      ),
    );
  }
}
