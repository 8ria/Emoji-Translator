import json
import numpy as np
from collections import Counter

EMOJI_JSON_PATH = "emoji.json"
GLOVE_PATH = "glove.txt"
OUTPUT_PATH = "emoji.txt"
VECTOR_DIM = 50

print("Loading GloVe...")
glove = {}
with open(GLOVE_PATH, "r", encoding="utf8") as f:
    for line in f:
        parts = line.strip().split()
        if len(parts) != VECTOR_DIM + 1:
            continue
        w = parts[0]
        if len(w) == 1 and not w.isalpha():
            continue
        glove[w] = np.array(parts[1:], dtype=np.float32)

print("Loading emoji.json...")
with open(EMOJI_JSON_PATH, "r", encoding="utf8") as f:
    emoji_data = json.load(f)

print("Computing keyword frequencies...")
all_keywords = [w.lower() for kws in emoji_data.values() for w in kws if w.lower() in glove]
freq = Counter(all_keywords)
total = sum(freq.values())
word_freq = {w: c / total for w, c in freq.items()}

epsilon = 1e-5
weight_func = lambda f: 1.0 / (f + epsilon)

print("Computing weighted emoji vectors...")
with open(OUTPUT_PATH, "w", encoding="utf8") as fout:
    count = 0
    for emoji, keywords in emoji_data.items():
        kws = [w.lower() for w in keywords if w.lower() in glove and w.lower() in word_freq]
        if not kws:
            continue
        weights = np.array([weight_func(word_freq[w]) for w in kws])
        weights /= weights.sum()
        vecs = np.array([glove[w] for w in kws])
        weighted_vec = np.average(vecs, axis=0, weights=weights)
        norm = np.linalg.norm(weighted_vec)
        if norm == 0:
            continue
        norm_vec = weighted_vec / norm
        fout.write(f"{emoji} {' '.join(f'{x:.6f}' for x in norm_vec)}\n")
        count += 1

print(f"Saved {count} emoji vectors to {OUTPUT_PATH}")
