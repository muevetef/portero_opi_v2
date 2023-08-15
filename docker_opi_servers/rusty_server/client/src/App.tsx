import { Navigate, Route, Routes } from 'react-router';
import CameraPage from './pages/Camera';
import MainLayout from './layouts/MainLayout';

const App = () => {
  return (
    <Routes>
      <Route path='/' element={<MainLayout/>}>
        <Route path='feed' element={<CameraPage/>}/>
        <Route path='users' element={<></>}/>
      </Route>

      <Route path='*' element={<Navigate to='/feed' />} />
    </Routes>
  )
}

export default App
