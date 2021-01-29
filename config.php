<?php

return [
    'production' => false,
    'baseUrl' => 'http://threedots.ca',
    'site_title' => 'three dots …',
    'author' => 'Alan Rempel',
    'excerpt_marker' => '<!-- FOLD -->',
    'collections' => [
      'top' => [
        'path' => '{filename}',
        'sort' => 'weight'
      ],
      'misc' => [
        'path' => '{filename}',
        'sort' => '-date_revised'
      ],
      'posts' => [
        'sort' => '-date'
      ]
    ],
];
