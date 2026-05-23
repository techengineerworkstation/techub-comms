import { View, Text, TouchableOpacity, StyleSheet, ScrollView } from 'react-native';
import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';

export default function HomeScreen() {
  const router = useRouter();

  const actions = [
    { label: 'Start Video Call', icon: '📹', route: '/video' as const, color: '#009999' },
    { label: 'Voice Call', icon: '📞', route: '/voice' as const, color: '#c4a06a' },
    { label: 'Send Message', icon: '💬', route: '/messages' as const, color: '#007a7a' },
  ];

  return (
    <ScrollView style={styles.container}>
      <StatusBar style="light" />
      <View style={styles.hero}>
        <Text style={styles.heroTitle}>Welcome to Techub</Text>
        <Text style={styles.heroSubtitle}>Connect through video, voice, and messaging</Text>
      </View>

      <View style={styles.actions}>
        {actions.map((action) => (
          <TouchableOpacity
            key={action.label}
            style={[styles.actionCard, { borderLeftColor: action.color }]}
            onPress={() => router.push(action.route)}
          >
            <Text style={styles.actionIcon}>{action.icon}</Text>
            <View style={styles.actionText}>
              <Text style={styles.actionLabel}>{action.label}</Text>
              <Text style={styles.actionHint}>Tap to start</Text>
            </View>
          </TouchableOpacity>
        ))}
      </View>

      <View style={styles.infoCard}>
        <Text style={styles.infoTitle}>Quick Join</Text>
        <Text style={styles.infoText}>
          Go to the Video tab and enter a room name to join or create a meeting instantly.
        </Text>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#fdf8f0' },
  hero: { backgroundColor: '#009999', padding: 32, paddingTop: 20 },
  heroTitle: { fontSize: 28, fontWeight: 'bold', color: '#ffffff', marginBottom: 8 },
  heroSubtitle: { fontSize: 16, color: '#b3e8e8' },
  actions: { padding: 16, gap: 12 },
  actionCard: {
    backgroundColor: '#ffffff',
    borderRadius: 12,
    padding: 20,
    flexDirection: 'row',
    alignItems: 'center',
    borderLeftWidth: 4,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 4,
    elevation: 2,
  },
  actionIcon: { fontSize: 32, marginRight: 16 },
  actionText: { flex: 1 },
  actionLabel: { fontSize: 18, fontWeight: '600', color: '#1a1a1a' },
  actionHint: { fontSize: 14, color: '#6b7280', marginTop: 2 },
  infoCard: {
    backgroundColor: '#e6f7f7',
    margin: 16,
    padding: 20,
    borderRadius: 12,
  },
  infoTitle: { fontSize: 18, fontWeight: '600', color: '#005c5c', marginBottom: 8 },
  infoText: { fontSize: 14, color: '#007a7a', lineHeight: 20 },
});
