import type { NCCOAction } from '../types';

export function talkAction(text: string, options?: {
  language?: string;
  voiceName?: string;
  premium?: boolean;
  loop?: number;
  bargeIn?: boolean;
}): NCCOAction {
  return {
    action: 'talk',
    text,
    language: options?.language || 'en-US',
    voiceName: options?.voiceName || 'Amy',
    premium: options?.premium || false,
    loop: options?.loop || 1,
    bargeIn: options?.bargeIn ?? true,
  };
}

export function streamAction(streamUrl: string[], options?: {
  loop?: number;
  bargeIn?: boolean;
}): NCCOAction {
  return {
    action: 'stream',
    streamUrl,
    loop: options?.loop || 1,
    bargeIn: options?.bargeIn ?? true,
  };
}

export function inputAction(options: {
  dtmf?: { maxDigits?: number; timeOut?: number; submitOnHash?: boolean };
  speech?: { language?: string; endOnSilence?: number; context?: string[]; startTimeout?: number; maxDuration?: number };
  eventUrl: string[];
}): NCCOAction {
  const action: NCCOAction = {
    action: 'input',
    type: [],
    eventUrl: options.eventUrl,
    eventMethod: 'POST',
  };

  if (options.dtmf) {
    (action.type as string[]).push('dtmf');
    action.dtmf = options.dtmf;
  }
  if (options.speech) {
    (action.type as string[]).push('speech');
    action.speech = options.speech;
  }

  return action;
}

export function recordAction(options?: {
  format?: string;
  timeOut?: number;
  endOnSilence?: number;
  endOnKey?: string;
  eventUrl?: string[];
}): NCCOAction {
  return {
    action: 'record',
    format: options?.format || 'mp3',
    timeOut: options?.timeOut || 60,
    endOnSilence: options?.endOnSilence || 3,
    endOnKey: options?.endOnKey || '#',
    eventUrl: options?.eventUrl || [],
    eventMethod: 'POST',
  };
}

export function connectAction(endpoint: { type: string; number?: string; uri?: string }, options?: {
  from?: string;
  timeout?: number;
  limit?: number;
}): NCCOAction {
  return {
    action: 'connect',
    from: options?.from || process.env.VONAGE_NUMBER,
    endpoint: [endpoint],
    timeout: options?.timeout || 30,
    limit: options?.limit || 7200,
    eventUrl: [`${process.env.BASE_URL}/webhooks/event`],
    eventMethod: 'POST',
  };
}

export function conversationAction(name: string, options?: {
  musicOnHoldUrl?: string[];
  startOnEnter?: boolean;
  endOnExit?: boolean;
  record?: boolean;
}): NCCOAction {
  return {
    action: 'conversation',
    name,
    musicOnHoldUrl: options?.musicOnHoldUrl || [],
    startOnEnter: options?.startOnEnter ?? true,
    endOnExit: options?.endOnExit ?? false,
    record: options?.record || false,
  };
}

export function ivrMenu(
  prompt: string,
  options: {
    eventUrl: string[];
    dtmfOptions?: { maxDigits?: number; timeOut?: number };
    language?: string;
    voiceName?: string;
  }
): NCCOAction[] {
  return [
    talkAction(prompt, {
      language: options.language,
      voiceName: options.voiceName,
      bargeIn: false,
    }),
    inputAction({
      dtmf: {
        maxDigits: options.dtmfOptions?.maxDigits || 1,
        timeOut: options.dtmfOptions?.timeOut || 5,
      },
      eventUrl: options.eventUrl,
    }),
  ];
}
