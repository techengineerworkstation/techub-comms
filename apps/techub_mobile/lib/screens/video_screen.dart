import 'package:flutter/material.dart';
import '../theme/app_theme.dart';

class VideoScreen extends StatelessWidget {
  const VideoScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Video Conferencing')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            // Video Grid
            Expanded(
              child: GridView.count(
                crossAxisCount: 2,
                mainAxisSpacing: 12,
                crossAxisSpacing: 12,
                children: [
                  _VideoTile(name: 'You', isLocal: true),
                  _VideoTile(name: 'Waiting...', isLocal: false),
                ],
              ),
            ),
            const SizedBox(height: 16),
            // Controls
            Container(
              padding: const EdgeInsets.symmetric(vertical: 12, horizontal: 16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: AppTheme.beige100),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  _ControlButton(icon: Icons.mic, label: 'Mute', isActive: true),
                  _ControlButton(icon: Icons.videocam, label: 'Camera', isActive: true),
                  _ControlButton(icon: Icons.screen_share, label: 'Share', isActive: false),
                  _ControlButton(icon: Icons.fiber_manual_record, label: 'Record', isActive: false, isDanger: false),
                  _ControlButton(icon: Icons.call_end, label: 'Leave', isActive: false, isDanger: true),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _VideoTile extends StatelessWidget {
  final String name;
  final bool isLocal;

  const _VideoTile({required this.name, required this.isLocal});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.grey[900],
        borderRadius: BorderRadius.circular(12),
      ),
      child: Stack(
        alignment: Alignment.center,
        children: [
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              CircleAvatar(
                radius: 32,
                backgroundColor: isLocal ? AppTheme.teal500 : AppTheme.beige500,
                child: Text(name[0], style: const TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
              ),
              const SizedBox(height: 8),
              Text(name, style: const TextStyle(color: Colors.white70, fontSize: 13)),
            ],
          ),
          Positioned(
            bottom: 8,
            left: 8,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(color: Colors.black54, borderRadius: BorderRadius.circular(8)),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (isLocal) Container(width: 6, height: 6, decoration: const BoxDecoration(color: Colors.green, shape: BoxShape.circle)),
                  if (isLocal) const SizedBox(width: 4),
                  Text(name, style: const TextStyle(color: Colors.white, fontSize: 11)),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _ControlButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final bool isActive;
  final bool isDanger;

  const _ControlButton({required this.icon, required this.label, this.isActive = false, this.isDanger = false});

  @override
  Widget build(BuildContext context) {
    final bgColor = isDanger
        ? Colors.red
        : isActive
            ? AppTheme.teal500
            : Colors.white;
    final fgColor = isDanger || isActive ? Colors.white : AppTheme.metallicDark;

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 48,
          height: 48,
          decoration: BoxDecoration(
            color: bgColor,
            borderRadius: BorderRadius.circular(12),
            border: isDanger || isActive ? null : Border.all(color: AppTheme.beige200),
            boxShadow: [BoxShadow(color: (isDanger ? Colors.red : AppTheme.teal500).withOpacity(0.2), blurRadius: 8)],
          ),
          child: Icon(icon, color: fgColor, size: 22),
        ),
        const SizedBox(height: 4),
        Text(label, style: TextStyle(fontSize: 10, color: Colors.grey[600])),
      ],
    );
  }
}
