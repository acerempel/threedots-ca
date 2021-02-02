<?php

return [
    'production' => false,
    'baseUrl' => 'http://threedots.ca',
    'site_title' => 'three dots …',
    'author' => 'Alan Rempel',
    'excerpt_marker' => '<!-- FOLD -->',
    'getIncipit' => function ($page, $content) {
      $punctuation = ['.',',',':',';','!','?','—','–'];
      $words = explode(' ', (strip_tags($content)));
      $max_words = 23;
      $num_words = $max_words;
      $haspunct = false;
      foreach ($words as $index => $word) {
        if ($index + 1 === $max_words) break;
        foreach ($punctuation as $punct) {
          $pos = strrpos($word, $punct);
          if ($pos) {
            $num_words = $index + 1;
            $words[$index] = str_split($word, $pos)[0];
            $haspunct = true;
          }
        }
        if ($haspunct) break;
      }
      return implode(' ', array_slice($words, 0, $num_words));
    },
    'collections' => [
      'top' => [
        'path' => function ($page) {
          return $page->permalink ?? $page->getFilename();
        },
        'sort' => 'weight'
      ],
      'misc' => [
        'path' => '{filename}',
        'sort' => '-date_revised'
      ],
      'posts' => [
        'sort' => '-date',
        'filter' => function ($post) {
          return $post->status !== 'hidden';
        }
      ]
    ],
];
