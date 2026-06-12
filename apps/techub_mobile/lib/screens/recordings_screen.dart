import 'package:flutter/material.dart';
import '../theme/app_theme.dart';

class RecordingsScreen extends StatelessWidget {
  const RecordingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Recordings')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            // Search
            Row(
              children: [
                Expanded(
                  child: TextField(
                    decoration: InputDecoration(
                      hintText: 'Search recordings...',
                      hintStyle: TextStyle(color: Colors.grey[400]),
                      prefixIcon: Icon(Icons.search, color: Colors.grey[400]),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                ElevatedButton(onPressed: () {}, child: const Text('Search')),
              ],
            ),
            const SizedBox(height: 20),

            // Empty state
            Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Container(
                      width: 80,
                      height: 80,
                      decoration: BoxDecoration(
                        color: AppTheme.beige100,
                        shape: BoxShape.circle,
                      ),
                      child: Icon(Icons.folder_open, size: 40, color: AppTheme.beige400),
                    ),
                    const SizedBox(height: 16),
                    const Text('No recordings yet', style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600)),
                    const SizedBox(height: 8),
                    Text('Start a meeting and record it to see recordings here', style: TextStyle(color: Colors.grey[500], fontSize: 14), textAlign: TextAlign.center),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
