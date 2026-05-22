import { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet, Alert } from 'react-native';
import { config } from './config';

export default function VideoScreen() {
  const [roomName, setRoomName] = useState('');
  const [isJoining, setIsJoining] = useState(false);

  const handleJoin = async () => {
    if (!roomName.trim()) {
      Alert.alert('Error', 'Please enter a room name');
      return;
    }

    setIsJoining(true);
    try {
      const res = await fetch(`${config.apiBaseUrl}/api/video/session/${roomName.trim()}`);
      if (!res.ok) throw new Error('Failed to create session');
      const data = await res.json();
      Alert.alert('Session Ready', `Session ID: ${data.sessionId}\nToken received. Ready to join!`);
    } catch (err: any) {
      Alert.alert('Error', err.message);
    } finally {
      setIsJoining(false);
    }
  };

  return (
    <View style={styles.container}>
      <View style={styles.card}>
        <Text style={styles.title}>Video Meeting</Text>
        <Text style={styles.subtitle}>Enter a room name to join or create a meeting</Text>

        <TextInput
          style={styles.input}
          placeholder="Room name (e.g., team-standup)"
          placeholderTextColor="#9ca3af"
          value={roomName}
          onChangeText={setRoomName}
          autoCapitalize="none"
        />

        <TouchableOpacity
          style={[styles.button, isJoining && styles.buttonDisabled]}
          onPress={handleJoin}
          disabled={isJoining}
        >
          <Text style={styles.buttonText}>{isJoining ? 'Joining...' : 'Join Meeting'}</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.features}>
        <Text style={styles.featuresTitle}>Video Features</Text>
        {['Real-time Video', 'Screen Sharing', 'Recording', 'Live Captions', 'Chat'].map((f) => (
          <View key={f} style={styles.featureItem}>
            <Text style={styles.featureDot}>●</Text>
            <Text style={styles.featureText}>{f}</Text>
          </View>
        ))}
      </View>
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
  button: {
    backgroundColor: '#009999',
    borderRadius: 8,
    padding: 16,
    alignItems: 'center',
  },
  buttonDisabled: { opacity: 0.6 },
  buttonText: { color: '#ffffff', fontSize: 16, fontWeight: '600' },
  features: {
    backgroundColor: '#e6f7f7',
    borderRadius: 12,
    padding: 20,
  },
  featuresTitle: { fontSize: 18, fontWeight: '600', color: '#005c5c', marginBottom: 12 },
  featureItem: { flexDirection: 'row', alignItems: 'center', marginBottom: 8 },
  featureDot: { color: '#009999', marginRight: 8, fontSize: 8 },
  featureText: { fontSize: 15, color: '#007a7a' },
});
