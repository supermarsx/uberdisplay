import './globals.css';

export const metadata = {
  title: 'UberDisplay Host',
  description: 'UberDisplay PC host app shell.'
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <div className="background">
          <div className="paper-veil" />
          {children}
        </div>
      </body>
    </html>
  );
}
