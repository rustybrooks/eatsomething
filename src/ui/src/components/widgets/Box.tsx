import './Box.css';

export function Box({ children, style = null }: { children: any; style?: any }) {
  return (
    <div className="Box" style={style}>
      {children}
    </div>
  );
}

export function SpanBox({ children, className = 'SpanBox', onClick = null }: { children: any; className: string; onClick?: any }) {
  return (
    <span className={className} onClick={onClick} style={{ cursor: onclick !== null ? 'pointer' : null }}>
      {children}
    </span>
  );
}
