import 'package:flutter/material.dart';
import '../theme/app_theme.dart';

class MessagesScreen extends StatefulWidget {
  const MessagesScreen({super.key});

  @override
  State<MessagesScreen> createState() => _MessagesScreenState();
}

class _MessagesScreenState extends State<MessagesScreen> {
  String _channel = 'sms';
  final _toController = TextEditingController();
  final _messageController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Messages')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Channel selector
            Row(
              children: [
                _ChannelChip(label: 'SMS', icon: Icons.email, isSelected: _channel == 'sms', onTap: () => setState(() => _channel = 'sms')),
                const SizedBox(width: 8),
                _ChannelChip(label: 'WhatsApp', icon: Icons.chat, isSelected: _channel == 'whatsapp', onTap: () => setState(() => _channel = 'whatsapp')),
                const SizedBox(width: 8),
                _ChannelChip(label: 'MMS', icon: Icons.image, isSelected: _channel == 'mms', onTap: () => setState(() => _channel = 'mms')),
              ],
            ),
            const SizedBox(height: 20),

            // Message form
            Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: AppTheme.beige100),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Send a Message', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
                  const SizedBox(height: 16),
                  TextField(
                    controller: _toController,
                    decoration: const InputDecoration(labelText: 'To', hintText: '+1 234 567 8901'),
                    keyboardType: TextInputType.phone,
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: _messageController,
                    decoration: const InputDecoration(labelText: 'Message', hintText: 'Type your message...'),
                    maxLines: 4,
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: ElevatedButton.icon(
                      onPressed: () {},
                      icon: const Icon(Icons.send),
                      label: Text('Send ${_channel.toUpperCase()}'),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _ChannelChip extends StatelessWidget {
  final String label;
  final IconData icon;
  final bool isSelected;
  final VoidCallback onTap;

  const _ChannelChip({required this.label, required this.icon, required this.isSelected, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
        decoration: BoxDecoration(
          gradient: isSelected ? AppTheme.turquoiseGradient : null,
          color: isSelected ? null : AppTheme.beige100,
          borderRadius: BorderRadius.circular(10),
          boxShadow: isSelected ? [BoxShadow(color: AppTheme.teal500.withOpacity(0.3), blurRadius: 8)] : null,
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 16, color: isSelected ? Colors.white : AppTheme.metallicDark),
            const SizedBox(width: 6),
            Text(label, style: TextStyle(color: isSelected ? Colors.white : AppTheme.metallicDark, fontWeight: FontWeight.w500, fontSize: 13)),
          ],
        ),
      ),
    );
  }
}
