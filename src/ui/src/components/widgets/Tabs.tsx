import './Tabs.css';

export function Tabs({ style = null, tabs = [], onChange = null, value }: { value: string; style?: any; tabs?: any[]; onChange?: any }) {
  return (
    <div className="tabs" style={style}>
      {tabs.map(t => (
        <button
          className="tabs"
          key={t[1]}
          tabIndex={t[1] === value ? 0 : -1}
          style={{ ...style, background: t[1] === value ? '#269CE9' : null, color: t[1] === value ? 'white' : null }}
          type="button"
          role="tab"
          aria-selected="true"
          onClick={() => onChange(t[1])}
        >
          {t[0]}
          <span key={t[1]} className="tabs" />
        </button>
      ))}
    </div>
  );
}
