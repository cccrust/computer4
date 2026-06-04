import { StatusBar } from './components/Shell/StatusBar'
import { HomeScreen } from './components/Shell/HomeScreen'
import { Dock } from './components/Shell/Dock'
import { AppContainer } from './components/Shell/AppContainer'
import { useAppStore } from './store/appStore'

function App() {
  const { viewState } = useAppStore()

  return (
    <div className="w-full h-full flex flex-col bg-os-bg overflow-hidden">
      <StatusBar />

      {viewState === 'home' ? (
        <>
          <HomeScreen />
          <Dock />
        </>
      ) : (
        <AppContainer />
      )}
    </div>
  )
}

export default App
