import System.Environment
import Criterion.Main

import Data.ByteString (ByteString)
import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as BSC

import Data.Trie (Trie)
import qualified Data.Trie as T

insertAll :: [ByteString] -> Trie Int
insertAll w = foldl insertSingle T.empty w
    where
        insertSingle trie str = T.insert str (BS.length str) trie

main = do
    -- Read the input file.
    filename <- getEnv "TEST_FILE"
    contents <- BS.readFile filename
    let testWords = BSC.words contents

    -- Run benchmarks.
    defaultMain [ bgroup "bytestring-trie" [ bench "insertAll" $ whnf insertAll testWords ] ]
