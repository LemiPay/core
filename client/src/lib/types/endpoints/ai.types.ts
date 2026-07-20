export interface ChatMessage {
	role: 'user' | 'assistant';
	content: string;
}

export interface AskRequest {
	question: string;
	history: ChatMessage[];
}

export interface AskResponse {
	answer: string;
}

export interface ExplainRequest {
	concept: string;
}

export interface ExplainResponse {
	explanation: string;
}
