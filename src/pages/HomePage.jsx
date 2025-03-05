import CardHome from '../components/CardHome';
import AppBarComponent from '../components/AppBarComponent';

function HomePage () {
  return (
    <body>
      <AppBarComponent />
      <div style={{position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', width: '75%', height: '75%' }}>
        <CardHome/>
      </div>
    </body>
  );
}
export default HomePage;