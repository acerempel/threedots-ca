---
title: Miscellaneous lists
link_text: Lists
description: A miscellany of lists.
weight: 24
---

@extends('_layouts.base')
@section('body')
  <section class="p-space-1/2">
    <h1 class="sans-serif semibold font-size-5">{{ $page->title }}</h1>
    <aside class="serif">Apart from their miscellany, these lists have one salient
      property in common: they revel in their incompleteness.</aside>
    @foreach ($misc as $misc_page)
      <p class="sans-serif">
        @link(['page' => $misc_page])
        @if ($misc_page->date_revised)
        <small class="light">— Updated @date($misc_page->date_revised)</small>
        @endif
      </p>
    @endforeach
  </section>
@endsection
