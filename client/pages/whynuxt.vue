<template>
<div>
<div class="term">

  <cat-command :name="this.$route.path"/>

 <div class="content">

  
  <nav-tab class="nav"/>

  <file-header :edited="edited" :name="this.$route.name" :created="20230124" />

  <div class="rawr">
    Why did I choose to build this website via NuxtJS?

        ~insert reason here~

  </div>
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
    title: 'whynuxt - berryez.xyz'
  },
  data: ()=>{
    return{
      edited: '20230114'
    }
  }
}

</script>

