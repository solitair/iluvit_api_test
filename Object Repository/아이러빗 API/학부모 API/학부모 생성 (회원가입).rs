<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>학부모 생성 (회원가입)</name>
   <tag></tag>
   <elementGuidId>4438edd2-e890-44c9-bf1f-8795685f59d6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;loginId\&quot;: \&quot;parent0765\&quot;,\n    \&quot;password\&quot;: \&quot;parent0765!\&quot;,\n    \&quot;passwordCheck\&quot;: \&quot;parent0765!\&quot;,\n    \&quot;phoneNum\&quot;: \&quot;${GlobalVariable.G_ParentPhoneNumber}\&quot;,\n    \&quot;nickname\&quot;: \&quot;아이러빗학부모4\&quot;,\n    \&quot;name\&quot;: \&quot;이서우\&quot;,\n    \&quot;emailAddress\&quot;: \&quot;asd@asd\&quot;,\n    \&quot;address\&quot;: \&quot;서울특별시 금천구 가산디지털2로 108 뉴티캐슬\&quot;,\n    \&quot;detailAddress\&quot;: \&quot;1207호\&quot;,\n    \&quot;theme\&quot;: {\n        \&quot;english\&quot;: true,\n        \&quot;foreigner\&quot;: true,\n        \&quot;clean\&quot;: true,\n        \&quot;buddhism\&quot;: false,\n        \&quot;christianity\&quot;: false,\n        \&quot;catholic\&quot;: false,\n        \&quot;animal\&quot;: false,\n        \&quot;plant\&quot;: false,\n        \&quot;camping\&quot;: false,\n        \&quot;nature\&quot;: false,\n        \&quot;art\&quot;: false,\n        \&quot;music\&quot;: false,\n        \&quot;math\&quot;: false,\n        \&quot;sport\&quot;: false,\n        \&quot;coding\&quot;: false,\n        \&quot;manner\&quot;: false,\n        \&quot;genius\&quot;: false\n    },\n    \&quot;interestAge\&quot;: 4\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>25ea854b-454d-4b87-a267-4d4dbf17e784</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.G_ServerUrl}/parent/signup</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
