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

getAll :: Trie Int -> [ByteString] -> [Maybe Int]
getAll trie keys = map (flip T.lookup trie) keys

removeAll :: Trie Int -> [ByteString] -> Trie Int
removeAll trie keys = foldl (flip T.delete) trie keys

main = do
    -- Read the input file.
    filename <- getEnv "TEST_FILE"
    contents <- BS.readFile filename
    let testWords = BSC.words contents

    let fullTrie = insertAll testWords

    -- Run benchmarks.
    defaultMain [
        bgroup "bytestring-trie" [
            bench "insert" $ whnf insertAll testWords,
            bench "get" $ nf (getAll fullTrie) testWords,
            bench "remove" $ whnf (removeAll fullTrie) testWords
            ]
        ]
