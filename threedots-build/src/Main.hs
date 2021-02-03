module Main (main) where

import Options.Applicative hiding (action)
import Development.Shake
import Development.Shake.FilePath
import Development.Shake.Rule
import Development.Shake.Classes
import GHC.Generics (Generic)
import Data.ByteString (ByteString)
import qualified System.Directory as IO

main :: IO ()
main = do
  options <- execParser cli
  shake (makeShakeOptions options) $ build options

makeShakeOptions :: Options -> ShakeOptions
makeShakeOptions options =
  shakeOptions
    { shakeRebuild = if rebuildAll options then [(RebuildNow, "**/*")] else []
    , shakeTimings = timings options
    , shakeVerbosity = verbosity options
    , shakeVersion = "2"
    }

rollupConfig = "rollup.config.js"
yarnLockfile = "yarn.lock"
sourceDirectory = "source" -- TODO make this configurable
jsSubdirectory = "_scripts"
jsBundle = sourceDirectory </> "assets/build/js/main.js"

build :: Options -> Rules ()
build options = do

  addBuiltinRule noLint noIdentity runSiteQ

  action $ parallel [need (targets options), apply1 (SiteQ Development)]

  jsBundle %> \_ -> do
    jsFiles <- getDirectoryFiles "" [sourceDirectory </> jsSubdirectory </> "*.js" ]
    need $ rollupConfig : yarnLockfile : jsFiles
    cmd_ (UserCommand "rollup -c") "yarn run rollup -c"

  yarnLockfile %> \_ -> do
    need ["package.json"]
    cmd_ (UserCommand "yarn install") "yarn install"

data SiteQ = SiteQ Mode
  deriving stock ( Eq, Show, Generic )
  deriving anyclass ( Hashable, Binary, NFData )

data Mode = Development | Production
  deriving stock ( Eq, Show, Generic )
  deriving anyclass ( Hashable, Binary, NFData )

runSiteQ :: SiteQ -> Maybe ByteString -> RunMode -> Action (RunResult ())
runSiteQ (SiteQ buildMode) _mbStored runMode = do

  let sourceFileExtensions = ["blade.php", "blade.xml", "blade.md", "md", "css"]
  inputFiles <- getDirectoryFiles "" (map ((sourceDirectory </> "**/*") <.>) sourceFileExtensions)
  let configFiles = case buildMode of
        Development -> ["config.php"]
        Production -> ["config.php", "config.production.php"]
  let phpFiles = ["blade.php", "bootstrap.php"]
  need $ jsBundle : configFiles ++ phpFiles ++ inputFiles

  let outputDirectory = case buildMode of
        Development -> "build_local"
        Production -> "build_production"
  outDirExists <- liftIO $ IO.doesDirectoryExist outputDirectory

  if | not outDirExists -> rebuild
     | RunDependenciesChanged <- runMode -> rebuild
     | otherwise -> don'tRebuild

 where
  rebuild = do
    let envArg = case buildMode of
          Development -> "local"
          Production -> "production"
    cmd_ (UserCommand "jigsaw build") "./vendor/bin/jigsaw build --cache --" envArg
    return $ RunResult ChangedRecomputeDiff mempty ()

  don'tRebuild =
    return $ RunResult ChangedNothing mempty ()

type instance RuleResult SiteQ = ()

data Options = Options
  { targets :: [String]
  , rebuildAll :: Bool
  , timings :: Bool
  , verbosity :: Verbosity
  }

optionsParser :: Parser Options
optionsParser = Options
  <$> many (strArgument (metavar "TARGET" <> help "Targets to build"))
  <*> switch (long "rebuild-all" <> short 'R' <> help "Rebuild everything regardless of whether dependencies have changed")
  <*> switch (long "timings" <> short 't' <> help "Print timings of internal operations after completion")
  <*> (flag' Warn (long "quiet" <> short 'q' <> help "Print only errors and warnings")
      <|> flag' Verbose (long "verbose" <> short 'v' <> help "Print various additional messages")
      <|> flag' Silent (long "silent" <> short 's' <> help "Print nothing")
      <|> pure Info)

cli :: ParserInfo Options
cli = info (optionsParser <**> helper) $ fullDesc <> progDesc "Build threedots.ca"
