<template>
<div>
<div class="term">

  <cat-command :name="this.$route.path"/>

 <div class="content">

  
  <nav-tab class="nav"/>

  <file-header :edited="edited" :name="this.$route.name" :created="20230124" />

  <div class="rawr">
        Instances:
    <ul>
      <ol>
        <a target="_blank" href="https://search.berryez.xyz/">SearXNG (Duckduckgo,Google)</a>
        <ol>"SearXNG is a metasearch engine, aggregating the results of other search engines while not storing information about its users."</ol>
      </ol>

      <ol>
        <a>Matrix (Synapse) | matrix.berryez.xyz </a>
        <ol><a href="https://matrix.org/" target="_blank">About matrix</a></ol>
      </ol>

      <ol>
        <a target="_blank" href="https://piped.berryez.xyz/">Piped</a>
        <ol>"An alternative privacy-friendly YouTube frontend which is efficient by design. "</ol>
      </ol>
    </ul>

  </div>
   <img src="/vps.png">
 </div>


</div>
<all-footer />
</div>


</template>

<script>

import CatCommand from '../components/catCommand.vue'
import FileHeader from '../components/fileHeader.vue'
import navTab from '../components/navTab.vue'
import allFooter from '../components/allFooter.vue'
export default {
  components: { navTab, FileHeader, CatCommand,allFooter },
  name: 'Index',
  head: {
    title: 'land - berryez.xyz'
  },
  data: ()=>{
    return{
      edited: '20230114'
    }
  }
}

</script>

