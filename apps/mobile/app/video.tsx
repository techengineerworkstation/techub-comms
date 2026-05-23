import { useState, useEffect, useRef } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  Alert,
  Dimensions,
} from 'react-native';
import {
  OTSession,
  OTPublisher,
  OTSubscriber,
  OTSessionEvent,
} from 'opentok-react-native';
import { config } from './config';

const { width } = Dimensions.get('window');

export default function VideoScreen() {
  const [roomName, setRoomName] = useState('');
  const [isJoining, setIsJoining] = useState(false);
  const [sessionData, setSessionData] = useState<{
    sessionId: string;
    token: string;
    apiKey: string;
  } | null>(null);
  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const sessionRef = useRef<any>(null);

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
      setSessionData(data);
    } catch (err: any) {
      Alert.alert('Error', err.message);
    } finally {
      setIsJoining(false);
    }
  };

  const handleLeave = () => {
    setSessionData(null);
    setIsConnected(false);
    setIsMuted(false);
    setIsVideoOff(false);
  };

  const handleSessionError = (event: OTSessionEvent) => {
    console.error('Session error:', event);
    Alert.alert('Connection Error', 'Failed to connect to the meeting.');
  };

  const handleStreamCreated = (event: any) => {
    console.log('Stream created:', event.streamId);
  };

  const handleStreamDestroyed = (event: any) => {
    console.log('Stream destroyed:', event.streamId);
  };

  if (!sessionData) {
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

  return (
    <View style={styles.videoContainer}>
      <OTSession
        ref={sessionRef}
        apiKey={sessionData.apiKey}
        sessionId={sessionData.sessionId}
        token={sessionData.token}
        onError={handleSessionError}
        eventHandlers={{
          streamCreated: handleStreamCreated,
          streamDestroyed: handleStreamDestroyed,
          sessionConnected: () => setIsConnected(true),
        }}
      >
        {/* Remote subscribers (full screen) */}
        <OTSubscriber
          style={styles.subscriberVideo}
          eventHandlers={{
            streamDestroyed: () => console.log('Subscriber stream destroyed'),
          }}
        />

        {/* Local publisher (small overlay) */}
        <View style={styles.publisherContainer}>
          <OTPublisher
            style={styles.publisherVideo}
            properties={{
              publishAudio: !isMuted,
              publishVideo: !isVideoOff,
              cameraPosition: 'front',
            }}
          />
        </View>

        {/* Connection status */}
        {!isConnected && (
          <View style={styles.connectingOverlay}>
            <Text style={styles.connectingText}>Connecting...</Text>
          </View>
        )}
      </OTSession>

      {/* Controls */}
      <View style={styles.controls}>
        <TouchableOpacity
          style={[styles.controlButton, isMuted && styles.controlButtonActive]}
          onPress={() => setIsMuted(!isMuted)}
        >
          <Text style={styles.controlText}>{isMuted ? '🔇' : '🔊'}</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={[styles.controlButton, isVideoOff && styles.controlButtonActive]}
          onPress={() => setIsVideoOff(!isVideoOff)}
        >
          <Text style={styles.controlText}>{isVideoOff ? '📷' : '📹'}</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={[styles.controlButton, styles.leaveButton]}
          onPress={handleLeave}
        >
          <Text style={styles.controlText}>📞</Text>
        </TouchableOpacity>
      </View>

      {/* Room info */}
      <View style={styles.roomInfo}>
        <Text style={styles.roomInfoText}>Room: {roomName}</Text>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fdf8f0',
    padding: 16,
  },
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
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 14,
    color: '#6b7280',
    marginBottom: 20,
  },
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
  buttonDisabled: {
    opacity: 0.6,
  },
  buttonText: {
    color: '#ffffff',
    fontSize: 16,
    fontWeight: '600',
  },
  features: {
    backgroundColor: '#e6f7f7',
    borderRadius: 12,
    padding: 20,
  },
  featuresTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#005c5c',
    marginBottom: 12,
  },
  featureItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 8,
  },
  featureDot: {
    color: '#009999',
    marginRight: 8,
    fontSize: 8,
  },
  featureText: {
    fontSize: 15,
    color: '#007a7a',
  },
  videoContainer: {
    flex: 1,
    backgroundColor: '#000000',
  },
  subscriberVideo: {
    flex: 1,
    width: '100%',
    height: '100%',
  },
  publisherContainer: {
    position: 'absolute',
    top: 16,
    right: 16,
    width: 120,
    height: 160,
    borderRadius: 12,
    overflow: 'hidden',
    borderWidth: 2,
    borderColor: '#ffffff',
    elevation: 5,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.3,
    shadowRadius: 4,
  },
  publisherVideo: {
    width: '100%',
    height: '100%',
  },
  connectingOverlay: {
    ...StyleSheet.absoluteFillObject,
    backgroundColor: 'rgba(0, 0, 0, 0.7)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  connectingText: {
    color: '#ffffff',
    fontSize: 18,
    fontWeight: '600',
  },
  controls: {
    position: 'absolute',
    bottom: 40,
    left: 0,
    right: 0,
    flexDirection: 'row',
    justifyContent: 'center',
    gap: 20,
  },
  controlButton: {
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: 'rgba(255, 255, 255, 0.2)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  controlButtonActive: {
    backgroundColor: 'rgba(239, 68, 68, 0.8)',
  },
  leaveButton: {
    backgroundColor: '#ef4444',
  },
  controlText: {
    fontSize: 24,
  },
  roomInfo: {
    position: 'absolute',
    top: 16,
    left: 16,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 8,
  },
  roomInfoText: {
    color: '#ffffff',
    fontSize: 14,
    fontWeight: '500',
  },
});
