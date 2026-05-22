import { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet, Alert } from 'react-native';
import { config } from './config';

export default function VoiceScreen() {
  const [phoneNumber, setPhoneNumber] = useState('');
  const [message, setMessage] = useState('');
  const [callActive, setCallActive] = useState(false);
  const [callUuid, setCallUuid] = useState<string | null>(null);

  const handleCall = async () => {
    if (!phoneNumber.trim()) {
      Alert.alert('Error', 'Please enter a phone number');
      return;
    }

    try {
      const res = await fetch(`${config.apiBaseUrl}/api/voice/call`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          to: phoneNumber,
          text: message || 'Hello from Techub Comms.',
        }),
      });
      const data = await res.json();
      setCallUuid(data.uuid);
      setCallActive(true);
      Alert.alert('Call Started', `Call UUID: ${data.uuid}`);
    } catch (err: any) {
      Alert.alert('Error', err.message);
    }
  };

  const handleHangup = async () => {
    if (!callUuid) return;
    try {
      await fetch(`${config.apiBaseUrl}/api/voice/call/${callUuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'hangup' }),
      });
      setCallActive(false);
      setCallUuid(null);
      Alert.alert('Call Ended', 'The call has been disconnected.');
    } catch (err: any) {
      Alert.alert('Error', err.message);
    }
  };

  return (
    <View style={styles.container}>
      <View style={styles.card}>
        <Text style={styles.title}>Voice Call</Text>
        <Text style={styles.subtitle}>Make a phone call with text-to-speech</Text>

        <TextInput
          style={styles.input}
          placeholder="Phone number (e.g., +1234567890)"
          placeholderTextColor="#9ca3af"
          value={phoneNumber}
          onChangeText={setPhoneNumber}
          keyboardType="phone-pad"
        />

        <TextInput
          style={[styles.input, styles.textArea]}
          placeholder="TTS Message (optional)"
          placeholderTextColor="#9ca3af"
          value={message}
          onChangeText={setMessage}
          multiline
          numberOfLines={3}
        />

        {callActive ? (
          <TouchableOpacity style={styles.hangupButton} onPress={handleHangup}>
            <Text style={styles.buttonText}>Hang Up</Text>
          </TouchableOpacity>
        ) : (
          <TouchableOpacity style={styles.callButton} onPress={handleCall}>
            <Text style={styles.buttonText}>Call</Text>
          </TouchableOpacity>
        )}
      </View>

      {callActive && (
        <View style={styles.activeCard}>
          <Text style={styles.activeTitle}>Call Active</Text>
          <Text style={styles.activeUuid}>UUID: {callUuid}</Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#fdf8f0', padding: 16 },
  card: {
    backgroundColor: '#ffffff',
    borderRadius: 12,
    padding: 24,
    marginBottom: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 4,
    elevation: 2,
  },
  title: { fontSize: 24, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 8 },
  subtitle: { fontSize: 14, color: '#6b7280', marginBottom: 20 },
  input: {
    borderWidth: 1,
    borderColor: '#ecdcb8',
    borderRadius: 8,
    padding: 14,
    fontSize: 16,
    marginBottom: 16,
    backgroundColor: '#ffffff',
  },
  textArea: { minHeight: 80, textAlignVertical: 'top' },
  callButton: {
    backgroundColor: '#009999',
    borderRadius: 8,
    padding: 16,
    alignItems: 'center',
  },
  hangupButton: {
    backgroundColor: '#ef4444',
    borderRadius: 8,
    padding: 16,
    alignItems: 'center',
  },
  buttonText: { color: '#ffffff', fontSize: 16, fontWeight: '600' },
  activeCard: {
    backgroundColor: '#e6f7f7',
    borderRadius: 12,
    padding: 20,
  },
  activeTitle: { fontSize: 18, fontWeight: '600', color: '#005c5c', marginBottom: 8 },
  activeUuid: { fontSize: 12, color: '#007a7a', fontFamily: 'monospace' },
});
