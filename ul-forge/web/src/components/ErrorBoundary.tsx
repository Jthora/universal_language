import { Component, type ReactNode, type ErrorInfo } from "react";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error("[UL-Forge] Render error:", error, info.componentStack);
  }

  handleRetry = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) return this.props.fallback;
      return (
        <div
          style={{
            padding: 24,
            color: "#f88",
            background: "#1a1a1a",
            borderRadius: 8,
            margin: 16,
            fontFamily: "monospace",
          }}
        >
          <h3 style={{ margin: "0 0 8px" }}>Render Error</h3>
          <p style={{ margin: "0 0 12px", fontSize: 13, opacity: 0.8 }}>
            {this.state.error?.message ?? "Unknown error"}
          </p>
          <button
            onClick={this.handleRetry}
            style={{
              padding: "6px 16px",
              cursor: "pointer",
              background: "#333",
              color: "#fff",
              border: "1px solid #555",
              borderRadius: 4,
            }}
          >
            Retry
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}
