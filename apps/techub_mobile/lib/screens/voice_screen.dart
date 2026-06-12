import 'package:flutter/material.dart';
import '../theme/app_theme.dart';

class VoiceScreen extends StatefulWidget {
  const VoiceScreen({super.key});

  @override
  State<VoiceScreen> createState() => _VoiceScreenState();
}

class _VoiceScreenState extends State<VoiceScreen> {
  final _phoneController = TextEditingController();
  final _textController = TextEditingController();
  bool _isInCall = false;
  bool _isMuted = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Voice Calls')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Make a Call
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
                  const Text('Make a Call', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
                  const SizedBox(height: 16),
                  TextField(
                    controller: _phoneController,
                    decoration: const InputDecoration(labelText: 'Phone Number', hintText: '+1 234 567 8901'),
                    keyboardType: TextInputType.phone,
                  ),
                  const SizedBox(height: 12),
                  TextField(
                    controller: _textController,
                    decoration: const InputDecoration(labelText: 'TTS Message', hintText: 'Enter text to speak...'),
                    maxLines: 3,
                  ),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      Expanded(child: ElevatedButton.icon(onPressed: () => setState(() => _isInCall = true), icon: const Icon(Icons.call), label: const Text('Call'))),
                      const SizedBox(width: 8),
                      Expanded(child: OutlinedButton.icon(onPressed: () {}, icon: const Icon(Icons.phone_in_talk), label: const Text('IVR'))),
                      const SizedBox(width: 8),
                      Expanded(child: OutlinedButton.icon(onPressed: () {}, icon: const Icon(Icons.group), label: const Text('Conference'))),
                    ],
                  ),
                ],
              ),
            ),

            if (_isInCall) ...[
              const SizedBox(height: 16),
              // Active Call
              Container(
                padding: const EdgeInsets.all(20),
                decoration: BoxDecoration(
                  color: Colors.white,
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: AppTheme.teal200),
                  boxShadow: [BoxShadow(color: AppTheme.teal500.withOpacity(0.1), blurRadius: 12)],
                ),
                child: Column(
                  children: [
                    Row(
                      children: [
                        Container(width: 10, height: 10, decoration: BoxDecoration(color: Colors.green, shape: BoxShape.circle, boxShadow: [BoxShadow(color: Colors.green.withOpacity(0.4), blurRadius: 6)])),
                        const SizedBox(width: 8),
                        const Text('Active Call', style: TextStyle(fontWeight: FontWeight.w600)),
                        const Spacer(),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                          decoration: BoxDecoration(gradient: AppTheme.turquoiseGradient, borderRadius: BorderRadius.circular(20)),
                          child: const Text('Connected', style: TextStyle(color: Colors.white, fontSize: 12)),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        _CallControl(icon: _isMuted ? Icons.volume_off : Icons.volume_up, label: _isMuted ? 'Unmute' : 'Mute', onTap: () => setState(() => _isMuted = !_isMuted), isActive: _isMuted),
                        _CallControl(icon: Icons.record_voice_over, label: 'TTS', onTap: () {}),
                        _CallControl(icon: Icons.call_end, label: 'Hang Up', onTap: () => setState(() => _isInCall = false), isDanger: true),
                      ],
                    ),
                    const SizedBox(height: 16),
                    // DTMF Keypad
                    const Text('DTMF Keypad', style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500, color: AppTheme.metallicDark)),
                    const SizedBox(height: 8),
                    GridView.count(
                      shrinkWrap: true,
                      physics: const NeverScrollableScrollPhysics(),
                      crossAxisCount: 4,
                      mainAxisSpacing: 8,
                      crossAxisSpacing: 8,
                      childAspectRatio: 1.5,
                      children: ['1','2','3','4','5','6','7','8','9','*','0','#'].map((d) =>
                        Container(
                          decoration: BoxDecoration(
                            gradient: const LinearGradient(colors: [AppTheme.beige100, AppTheme.beige200]),
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Center(child: Text(d, style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'monospace'))),
                        ),
                      ).toList(),
                    ),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

class _CallControl extends StatelessWidget {
  final IconData icon;
  final String label;
  final VoidCallback onTap;
  final bool isActive;
  final bool isDanger;

  const _CallControl({required this.icon, required this.label, required this.onTap, this.isActive = false, this.isDanger = false});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Column(
        children: [
          Container(
            width: 56,
            height: 56,
            decoration: BoxDecoration(
              color: isDanger ? Colors.red : isActive ? Colors.red.shade100 : AppTheme.beige100,
              borderRadius: BorderRadius.circular(16),
            ),
            child: Icon(icon, color: isDanger ? Colors.white : isActive ? Colors.red : AppTheme.metallicDark),
          ),
          const SizedBox(height: 4),
          Text(label, style: TextStyle(fontSize: 11, color: isDanger ? Colors.red : Colors.grey[600])),
        ],
      ),
    );
  }
}
