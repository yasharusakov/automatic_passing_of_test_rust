import {FormEvent, useState} from 'react'
import {invoke} from '@tauri-apps/api'
import githubSvg from '@svgs/github.svg'
import './style.scss'

function App() {
    const [username, setUsername] = useState<string>('')
    const [code, setCode] = useState<string>('')
    const [sourceAnswersUrl, setSourceAnswersUrl] = useState<string>('')

    const launchWebDriver = async (e: FormEvent) => {
        e.preventDefault()
        if (!username || !code || !sourceAnswersUrl) return
        if (code.length !== 7) return

        await invoke('launch_web_driver', {username, code, sourceAnswersUrl})
    }

    const authorPage = async () => {
        await invoke('author_page')
    }

    return (
        <form onSubmit={launchWebDriver}>
            <input
                placeholder="Enter username"
                onChange={(e) => setUsername(e.target.value)} value={username}
                type="text"
            />
            <input
                placeholder="Enter code"
                onChange={(e) => setCode(e.target.value)}
                value={code}
                type="text"
            />
            <input
                placeholder="Enter source answers url"
                onChange={(e) => setSourceAnswersUrl(e.target.value)}
                value={sourceAnswersUrl} type="text"
            />
            <button type="submit">Start passing the test</button>
            <button type="button" onClick={authorPage} className="author-info">
                <img src={githubSvg} alt="GitHub svg" />
                <div>yasharusakov</div>
            </button>
        </form>
    )
}

export default App