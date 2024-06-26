#!/bin/bash

set -ex

xmlstarlet ed --inplace \
    --update '//_:function[@c:identifier="xfconf_g_property_bind"]//_:parameter[@name="object"]//_:type/@name' -v "GObject.Object" \
    --update '//_:function[@c:identifier="xfconf_g_property_bind"]//_:parameter[@name="object"]//_:type/@c:type' -v "GObject*" \
    --update '//_:function[@c:identifier="xfconf_g_property_bind_gdkcolor"]//_:parameter[@name="object"]//_:type/@name' -v "GObject.Object" \
    --update '//_:function[@c:identifier="xfconf_g_property_bind_gdkcolor"]//_:parameter[@name="object"]//_:type/@c:type' -v "GObject*" \
    --update '//_:function[@c:identifier="xfconf_g_property_bind_gdkrgba"]//_:parameter[@name="object"]//_:type/@name' -v "GObject.Object" \
    --update '//_:function[@c:identifier="xfconf_g_property_bind_gdkrgba"]//_:parameter[@name="object"]//_:type/@c:type' -v "GObject*" \
    --update '//_:function[@c:identifier="xfconf_g_property_unbind_by_property"]//_:parameter[@name="object"]//_:type/@name' -v "GObject.Object" \
    --update '//_:function[@c:identifier="xfconf_g_property_unbind_by_property"]//_:parameter[@name="object"]//_:type/@c:type' -v "GObject*" \
    --update '//_:function[@c:identifier="xfconf_g_property_unbind_all"]//_:parameter[@name="channel_or_object"]//_:type/@name' -v "GObject.Object" \
    --update '//_:function[@c:identifier="xfconf_g_property_unbind_all"]//_:parameter[@name="channel_orobject"]//_:type/@c:type' -v "GObject*" \
    --append '//_:method[@c:identifier="xfconf_channel_get_property"]//_:parameter[@name="value" and not(@direction)]' -type attr -n "direction" -v "out" \
    Xfconf-0.gir
