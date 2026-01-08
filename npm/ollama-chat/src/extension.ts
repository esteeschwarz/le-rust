// extension.ts
import * as vscode from 'vscode';

interface OllamaMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

interface OllamaChatRequest {
  model: string;
  messages: OllamaMessage[];
  stream: boolean;
  options?: {
    temperature?: number;
    top_p?: number;
  };
}

interface OllamaStreamResponse {
  model: string;
  message: {
    role: string;
    content: string;
  };
  done: boolean;
}

export function activate(context: vscode.ExtensionContext) {
  // Register the chat participant
  const ollamaChat = vscode.chat.createChatParticipant('ollama.chat', async (
    request: vscode.ChatRequest,
    context: vscode.ChatContext,
    stream: vscode.ChatResponseStream,
    token: vscode.CancellationToken
  ) => {
    try {
      // Get configuration
      const config = vscode.workspace.getConfiguration('ollama');
      const baseUrl = config.get<string>('baseUrl', 'http://localhost:11434');
      const model = config.get<string>('model', 'llama2');
      const temperature = config.get<number>('temperature', 0.7);

      // Handle slash commands
      if (request.command) {
        return handleSlashCommand(request, context, stream, token, baseUrl, model);
      }

      // Build context from VS Code
      const messages = await buildMessages(request, context);

      // Stream response from Ollama
      await streamOllamaResponse(
        baseUrl,
        model,
        messages,
        temperature,
        stream,
        token
      );

    } catch (error) {
      stream.markdown(`❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  });

  // Add follower
  ollamaChat.followupProvider = {
    provideFollowups(result, context, token) {
      return [
        {
          prompt: 'Explain this in more detail',
          label: '📖 More details'
        },
        {
          prompt: 'Show me an example',
          label: '💡 Example'
        },
        {
          prompt: 'What are the alternatives?',
          label: '🔄 Alternatives'
        }
      ];
    }
  };

  // Register slash commands
  ollamaChat.iconPath = vscode.Uri.file(context.asAbsolutePath('icon.png'));
  
  context.subscriptions.push(ollamaChat);

  // Register configuration command
  context.subscriptions.push(
    vscode.commands.registerCommand('ollama.configure', async () => {
      const config = vscode.workspace.getConfiguration('ollama');
      
      const baseUrl = await vscode.window.showInputBox({
        prompt: 'Enter Ollama base URL',
        value: config.get('baseUrl', 'http://localhost:11434'),
        placeHolder: 'http://localhost:11434'
      });

      if (baseUrl) {
        await config.update('baseUrl', baseUrl, vscode.ConfigurationTarget.Global);
      }

      // Fetch and select model
      const models = await fetchAvailableModels(baseUrl || 'http://localhost:11434');
      if (models.length > 0) {
        const selectedModel = await vscode.window.showQuickPick(models, {
          placeHolder: 'Select Ollama model'
        });
        
        if (selectedModel) {
          await config.update('model', selectedModel, vscode.ConfigurationTarget.Global);
        }
      }

      vscode.window.showInformationMessage('Ollama configuration updated!');
    })
  );
}

async function buildMessages(
  request: vscode.ChatRequest,
  context: vscode.ChatContext
): Promise<OllamaMessage[]> {
  const messages: OllamaMessage[] = [];

  // Add system message
  messages.push({
    role: 'system',
    content: 'You are a helpful AI coding assistant integrated into VS Code. Provide concise, accurate answers about programming, code, and development.'
  });

  // Add conversation history
  for (const turn of context.history) {
    if (turn instanceof vscode.ChatRequestTurn) {
      messages.push({
        role: 'user',
        content: turn.prompt
      });
    } else if (turn instanceof vscode.ChatResponseTurn) {
      const content = turn.response.map(r => {
        if (r instanceof vscode.ChatResponseMarkdownPart) {
          return r.value.value;
        }
        return '';
      }).join('\n');
      
      messages.push({
        role: 'assistant',
        content
      });
    }
  }

  // Add current request with context
  let userPrompt = request.prompt;

  // Add references (files, selections, etc.)
  for (const ref of request.references) {
    if (ref.value instanceof vscode.Uri) {
      const doc = await vscode.workspace.openTextDocument(ref.value);
      userPrompt += `\n\nFile: ${ref.value.fsPath}\n\`\`\`\n${doc.getText()}\n\`\`\``;
    } else if (ref.value instanceof vscode.Location) {
      const doc = await vscode.workspace.openTextDocument(ref.value.uri);
      const text = doc.getText(ref.value.range);
      userPrompt += `\n\nCode from ${ref.value.uri.fsPath}:\n\`\`\`\n${text}\n\`\`\``;
    }
  }

  messages.push({
    role: 'user',
    content: userPrompt
  });

  return messages;
}

async function streamOllamaResponse(
  baseUrl: string,
  model: string,
  messages: OllamaMessage[],
  temperature: number,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken
): Promise<void> {
  const requestBody: OllamaChatRequest = {
    model,
    messages,
    stream: true,
    options: {
      temperature
    }
  };

  const response = await fetch(`${baseUrl}/api/chat`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(requestBody)
  });

  if (!response.ok) {
    throw new Error(`Ollama API error: ${response.statusText}`);
  }

  const reader = response.body?.getReader();
  const decoder = new TextDecoder();

  if (!reader) {
    throw new Error('No response body');
  }

  while (true) {
    if (token.isCancellationRequested) {
      reader.cancel();
      break;
    }

    const { done, value } = await reader.read();
    if (done) break;

    const chunk = decoder.decode(value);
    const lines = chunk.split('\n').filter(line => line.trim());

    for (const line of lines) {
      try {
        const data: OllamaStreamResponse = JSON.parse(line);
        if (data.message?.content) {
          stream.markdown(data.message.content);
        }
      } catch (e) {
        // Skip invalid JSON lines
      }
    }
  }
}

async function handleSlashCommand(
  request: vscode.ChatRequest,
  context: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
  baseUrl: string,
  model: string
): Promise<void> {
  const command = request.command;

  switch (command) {
    case 'explain':
      await handleExplainCommand(request, stream, baseUrl, model, token);
      break;
    case 'fix':
      await handleFixCommand(request, stream, baseUrl, model, token);
      break;
    case 'test':
      await handleTestCommand(request, stream, baseUrl, model, token);
      break;
    case 'doc':
      await handleDocCommand(request, stream, baseUrl, model, token);
      break;
    default:
      stream.markdown(`Unknown command: /${command}`);
  }
}

async function handleExplainCommand(
  request: vscode.ChatRequest,
  stream: vscode.ChatResponseStream,
  baseUrl: string,
  model: string,
  token: vscode.CancellationToken
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.document.getText(editor.selection);

  const messages: OllamaMessage[] = [
    {
      role: 'system',
      content: 'You are a code explainer. Provide clear, detailed explanations of code.'
    },
    {
      role: 'user',
      content: `Explain this code:\n\`\`\`\n${selection || request.prompt}\n\`\`\``
    }
  ];

  await streamOllamaResponse(baseUrl, model, messages, 0.7, stream, token);
}

async function handleFixCommand(
  request: vscode.ChatRequest,
  stream: vscode.ChatResponseStream,
  baseUrl: string,
  model: string,
  token: vscode.CancellationToken
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.document.getText(editor.selection);

  const messages: OllamaMessage[] = [
    {
      role: 'system',
      content: 'You are a code debugger. Identify issues and provide fixed code.'
    },
    {
      role: 'user',
      content: `Find and fix issues in this code:\n\`\`\`\n${selection || request.prompt}\n\`\`\``
    }
  ];

  await streamOllamaResponse(baseUrl, model, messages, 0.7, stream, token);
}

async function handleTestCommand(
  request: vscode.ChatRequest,
  stream: vscode.ChatResponseStream,
  baseUrl: string,
  model: string,
  token: vscode.CancellationToken
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.document.getText(editor.selection);

  const messages: OllamaMessage[] = [
    {
      role: 'system',
      content: 'You are a test generator. Create comprehensive unit tests.'
    },
    {
      role: 'user',
      content: `Generate unit tests for this code:\n\`\`\`\n${selection || request.prompt}\n\`\`\``
    }
  ];

  await streamOllamaResponse(baseUrl, model, messages, 0.7, stream, token);
}

async function handleDocCommand(
  request: vscode.ChatRequest,
  stream: vscode.ChatResponseStream,
  baseUrl: string,
  model: string,
  token: vscode.CancellationToken
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.document.getText(editor.selection);

  const messages: OllamaMessage[] = [
    {
      role: 'system',
      content: 'You are a documentation generator. Create clear, professional documentation.'
    },
    {
      role: 'user',
      content: `Generate documentation for this code:\n\`\`\`\n${selection || request.prompt}\n\`\`\``
    }
  ];

  await streamOllamaResponse(baseUrl, model, messages, 0.7, stream, token);
}

async function fetchAvailableModels(baseUrl: string): Promise<string[]> {
  try {
    const response = await fetch(`${baseUrl}/api/tags`);
    const data = await response.json();
    return data.models?.map((m: any) => m.name) || [];
  } catch {
    return ['llama2', 'codellama', 'mistral'];
  }
}

export function deactivate() {}