@extends('_layouts.base')
@section('body')
<article class="serif prose p-space-1/2">
  <header><h1 class="font-size-4.5 bold sans-serif">{{ $page->title }}</h1></header>
  @yield('content')
</article>
@endsection
