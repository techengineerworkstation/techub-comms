import { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet, Alert, ScrollView } from 'react-native';
import { config } from './config';

export default function MessagesScreen() {
  const [to, setTo] = useState('');
  const [message, setMessage] = useState('');
  const [channel, setChannel] = useState<'sms' | 'whatsapp'>('sms');
  const [sending, setSending] = useState(false);

  const handleSend = async () => {
    if (!to.trim() || !message.trim()) {
      Alert.alert('Error', 'Please enter a recipient and message');
      return;
    }

    setSending(true);
    try {
      const endpoint = channel === 'whatsapp' ? '/api/message/send-whatsapp' : '/api/message/send';
      const res = await fetch(`${config.apiBaseUrl}${endpoint}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ to, text: message }),
      });
      const data = await res.json();
      Alert.alert('Sent!', `Message ID: ${data.messageId}`);
      setMessage('');
    } catch (err: any) {
      Alert.alert('Error', err.message);
    } finally {
      setSending(false);
    }
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.card}>
        <Text style={styles.title}>Send Message</Text>

        <View style={styles.channelRow}>
          {(['sms', 'whatsapp'] as const).map((ch) => (
            <TouchableOpacity
              key={ch}
              style={[styles.channelBtn, channel === ch && styles.channelBtnActive]}
              onPress={() => setChannel(ch)}
            >
              <Text style={[styles.channelText, channel === ch && styles.channelTextActive]}>
                {ch.toUpperCase()}
              </Text>
            </TouchableOpacity>
          ))}
        </View>

        <TextInput
          style={styles.input}
          placeholder="Recipient phone number"
          placeholderTextColor="#9ca3af"
          value={to}
          onChangeText={setTo}
          keyboardType="phone-pad"
        />

        <TextInput
          style={[styles.input, styles.textArea]}
          placeholder="Type your message..."
          placeholderTextColor="#9ca3af"
          value={message}
          onChangeText={setMessage}
          multiline
          numberOfLines={4}
        />

        <TouchableOpacity
          style={[styles.sendButton, sending && styles.buttonDisabled]}
          onPress={handleSend}
          disabled={sending}
        >
          <Text style={styles.buttonText}>{sending ? 'Sending...' : `Send ${channel.toUpperCase()}`}</Text>
        </TouchableOpacity>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#fdf8f0', padding: 16 },
  card: {
    backgroundColor: '#ffffff',
    borderRadius: 12,
    padding: 24,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 4,
    elevation: 2,
  },
  title: { fontSize: 24, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 20 },
  channelRow: { flexDirection: 'row', gap: 8, marginBottom: 16 },
  channelBtn: {
    paddingHorizontal: 20,
    paddingVertical: 10,
    borderRadius: 8,
    backgroundColor: '#f5ead6',
  },
  channelBtnActive: { backgroundColor: '#009999' },
  channelText: { fontSize: 14, fontWeight: '600', color: '#6b7280' },
  channelTextActive: { color: '#ffffff' },
  input: {
    borderWidth: 1,
    borderColor: '#ecdcb8',
    borderRadius: 8,
    padding: 14,
    fontSize: 16,
    marginBottom: 16,
    backgroundColor: '#ffffff',
  },
  textArea: { minHeight: 100, textAlignVertical: 'top' },
  sendButton: {
    backgroundColor: '#009999',
    borderRadius: 8,
    padding: 16,
    alignItems: 'center',
  },
  buttonDisabled: { opacity: 0.6 },
  buttonText: { color: '#ffffff', fontSize: 16, fontWeight: '600' },
});
